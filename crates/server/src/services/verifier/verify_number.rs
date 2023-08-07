// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_auth::{AuthRequest, AuthResult};
use base::karma_coin::karma_coin_core_types::{User, UserVerificationData, VerificationResult};
use base::karma_coin::karma_coin_verifier::VerifyNumberRequest;
use base::signed_trait::SignedTrait;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<UserVerificationData>")]
pub(crate) struct Verify(pub VerifyNumberRequest);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<Verify> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: Verify,
    ) -> Result<UserVerificationData> {
        let req = msg.0;

        info!("verify phone number called");

        if self.auth_client.is_none() {
            return Err(anyhow!("internal error - auth client not initialized"));
        }

        // verify request signature
        if req.verify_signature().is_err() {
            return self.gen_result(VerificationResult::InvalidSignature).await;
        };

        let account_id = match req.account_id {
            Some(id) => id,
            None => {
                return self.gen_result(VerificationResult::MissingData).await;
            }
        };

        let phone_number = match req.mobile_number {
            Some(n) => n,
            None => {
                return self.gen_result(VerificationResult::InvalidSignature).await;
            }
        };

        info!("Phone num: {}", phone_number.number);

        let requested_user_name = req.requested_user_name.clone();

        if requested_user_name.is_empty() {
            return self.gen_result(VerificationResult::MissingData).await;
        }

        // check if there's a user for the accountId
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            // An existing user is asking to update his mobile number
            let user = User::decode(user_data.0.as_ref())?;
            if user.user_name != requested_user_name {
                // don't allow giving evidence on requested user name in case of existing user
                return self.gen_result(VerificationResult::UserNameTaken).await;
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
                return self.gen_result(VerificationResult::UserNameTaken).await;
            }
        }

        // todo: call auth service
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
                        return self.gen_result(VerificationResult::AccountMismatch).await
                    }
                    AuthResult::UserNotFound => {
                        return self.gen_result(VerificationResult::Unverified).await
                    }
                    AuthResult::UserAuthenticated => info!("user phone and account id verifier"),
                }
            }
            Err(e) => {
                return Err(anyhow!(
                    "internal error - auth service call failed: {:?}",
                    e
                ));
            }
        }

        // create signed verified response and return it
        let key_pair = self.get_key_pair().await?.to_ed2559_keypair();

        let mut resp = UserVerificationData::from(VerificationResult::Verified);

        // signed attestation details - user account id, nickname and verified mobile number
        resp.account_id = Some(account_id);
        resp.verifier_account_id = Some(self.get_account_id().await?);
        resp.requested_user_name = requested_user_name;
        resp.mobile_number = Some(phone_number);
        resp.signature = Some(resp.sign(&key_pair)?);
        info!("Returning verification response");
        Ok(resp)
    }
}

impl VerifierService {
    /// private helper function to generate a signed user verification data
    pub(crate) async fn gen_result(
        &mut self,
        value: VerificationResult,
    ) -> Result<UserVerificationData> {
        let mut data = UserVerificationData::from(value);
        let verifier_key_pair = self.get_key_pair().await?.to_ed2559_keypair();
        data.verifier_account_id = Some(self.get_account_id().await?);
        data.signature = Some(data.sign(&verifier_key_pair)?);
        Ok(data)
    }
}
