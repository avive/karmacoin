// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_auth::{AuthRequest, AuthResult};
use base::karma_coin::karma_coin_core_types::{User, UserVerificationDataEx, VerificationResult};
use base::karma_coin::karma_coin_verifier::{
    VerifyNumberRequestDataEx, VerifyNumberRequestEx, VerifyNumberResponseEx,
};
use base::server_config_service::{ServerConfigService, AUTH_SERVICE_BYPASS_KEY};
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use ed25519_dalek::{Signer, Verifier};
use prost::Message;
use uint::hex;
use xactor::*;

#[message(result = "Result<VerifyNumberResponseEx>")]
pub(crate) struct VerifyEx(pub VerifyNumberRequestEx);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<VerifyEx> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: VerifyEx,
    ) -> Result<VerifyNumberResponseEx> {
        let req = msg.0;

        info!("verify phone number ex called");

        if self.auth_client.is_none() {
            return Err(anyhow!("internal error - auth client not initialized"));
        }

        use ed25519_dalek::ed25519::signature::Signature;
        let signature = &Signature::from_bytes(&req.signature.as_ref()).unwrap();
        let message = &req.data;
        let pub_key = &ed25519_dalek::PublicKey::from_bytes(req.public_key.as_ref()).unwrap();

        // verify request signature
        if pub_key.verify(message, signature).is_err() {
            return self
                .gen_result_ex(VerificationResult::InvalidSignature)
                .await;
        };

        // create VerifyNumberRequestDataEx from data
        let data = VerifyNumberRequestDataEx::decode(req.data.as_ref())?;

        let account_id = match data.account_id {
            Some(id) => id,
            None => {
                return self.gen_result_ex(VerificationResult::MissingData).await;
            }
        };

        if !req.public_key.eq(&account_id.data) {
            // request public key used to verify signature doesn't match provided account id
            return self
                .gen_result_ex(VerificationResult::AccountMismatch)
                .await;
        }

        let phone_number = match data.mobile_number {
            Some(n) => n,
            None => {
                return self
                    .gen_result_ex(VerificationResult::InvalidSignature)
                    .await;
            }
        };

        info!("Phone num: {}", phone_number.number);

        let requested_user_name = data.requested_user_name.clone();

        if requested_user_name.is_empty() {
            return self.gen_result_ex(VerificationResult::MissingData).await;
        }

        // check if there's a user for the accountId
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(requested_user_name.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            // An existing user is asking to update his mobile number
            let user = User::decode(user_data.0.as_ref())?;
            if user.user_name != requested_user_name {
                // don't allow giving evidence on requested user name in case of existing user
                return self.gen_result_ex(VerificationResult::UserNameTaken).await;
            }
        } else {
            // verify that the requested username not already registered to another user
            let user_name_key = Bytes::from(requested_user_name.as_bytes().to_vec());
            if (DatabaseService::read(ReadItem {
                key: user_name_key.clone(),
                cf: USERS_NAMES_COL_FAMILY,
            })
            .await?)
                .is_some()
            {
                return self.gen_result_ex(VerificationResult::UserNameTaken).await;
            }
        }

        let bypass_token = hex::decode(
            ServerConfigService::get(AUTH_SERVICE_BYPASS_KEY.into())
                .await?
                .unwrap(),
        )
        .unwrap();

        if !data.bypass_token.eq(&bypass_token) {
            // call auth service ubless bypass token was provided
            match self
                .auth_client
                .as_mut()
                .unwrap()
                .authenticate(AuthRequest {
                    account_id: Some(account_id.clone()),
                    phone_number: phone_number.number.clone(),
                })
                .await
            {
                Ok(resp) => {
                    let res = AuthResult::from_i32(resp.into_inner().result);
                    if res.is_none() {
                        return Err(anyhow!(
                            "internal error - auth service returned an invalid result"
                        ));
                    }

                    match res.unwrap() {
                        AuthResult::AccountIdMismatch => {
                            return self
                                .gen_result_ex(VerificationResult::AccountMismatch)
                                .await
                        }
                        AuthResult::UserNotFound => {
                            return self.gen_result_ex(VerificationResult::Unverified).await
                        }
                        AuthResult::UserAuthenticated => {
                            info!("user phone and account id verifier")
                        }
                    }
                }
                Err(e) => {
                    return Err(anyhow!(
                        "internal error - auth service call failed: {:?}",
                        e
                    ));
                }
            }
        }

        // create signed verified response and return it
        let key_pair = self.get_key_pair().await?.to_ed2559_keypair();

        let mut resp = UserVerificationDataEx::from(VerificationResult::Verified);

        // signed attestation details - user account id, nickname and verified mobile number
        resp.account_id = Some(account_id);
        resp.verifier_account_id = Some(self.get_account_id().await?);
        resp.requested_user_name = requested_user_name;
        resp.mobile_number = Some(phone_number);

        let mut buf = Vec::with_capacity(resp.encoded_len());
        resp.encode(&mut buf)?;

        Ok(VerifyNumberResponseEx {
            verification_data: buf.to_vec(),
            signature: key_pair.sign(&buf.to_vec()).as_ref().to_vec(),
        })
    }
}

impl VerifierService {
    /// private helper function to generate a signed user verification data
    async fn gen_result_ex(&mut self, value: VerificationResult) -> Result<VerifyNumberResponseEx> {
        let mut data = UserVerificationDataEx::from(value);

        let verifier_key_pair = self.get_key_pair().await?.to_ed2559_keypair();
        data.verifier_account_id = Some(self.get_account_id().await?);

        let mut buf = Vec::with_capacity(data.encoded_len());
        data.encode(&mut buf)?;

        Ok(VerifyNumberResponseEx {
            verification_data: buf.to_vec(),
            signature: verifier_key_pair.sign(&buf.to_vec()).as_ref().to_vec(),
        })
    }
}
