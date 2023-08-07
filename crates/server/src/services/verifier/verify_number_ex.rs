// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::{User, UserVerificationData, VerificationResult};
use base::karma_coin::karma_coin_verifier::{VerifyNumberRequestDataEx, VerifyNumberRequestEx};
use base::server_config_service::{ServerConfigService, AUTH_SERVICE_BYPASS_KEY};
use base::signed_trait::SignedTrait;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use ed25519_dalek::Verifier;
use http::{header, StatusCode};
use prost::Message;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use xactor::*;

#[message(result = "Result<UserVerificationData>")]
pub(crate) struct VerifyEx(pub VerifyNumberRequestEx);

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
    pub sid: String,
}
/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<VerifyEx> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: VerifyEx,
    ) -> Result<UserVerificationData> {
        let req = msg.0;

        info!("verify phone number ex called");

        // decode request data
        let user_data = match VerifyNumberRequestDataEx::decode(req.data.as_ref()) {
            Ok(user_data) => user_data,
            Err(_) => {
                return self.gen_result(VerificationResult::MissingData).await;
            }
        };

        let account_id = match user_data.account_id {
            Some(id) => id.clone(),
            None => {
                return self.gen_result(VerificationResult::MissingData).await;
            }
        };

        // verify request signature
        use ed25519_dalek::ed25519::signature::Signature;
        let signature = &Signature::from_bytes(req.signature.as_ref()).unwrap();
        let pub_key = &ed25519_dalek::PublicKey::from_bytes(account_id.data.as_ref()).unwrap();

        // verify request data signature
        if pub_key.verify(req.data.as_ref(), signature).is_err() {
            return self.gen_result(VerificationResult::InvalidSignature).await;
        };

        // verify provided Twilio code
        // let code = user_data.

        let requested_user_name = user_data.requested_user_name.clone();

        if requested_user_name.is_empty() {
            return self.gen_result(VerificationResult::MissingData).await;
        }

        let phone_number = match user_data.mobile_number {
            Some(n) => n,
            None => {
                return self.gen_result(VerificationResult::MissingData).await;
            }
        };

        info!("Phone num: {}", phone_number.number);

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
                // don't allow giving evidence on new requested user name in case of existing user
                return self.gen_result(VerificationResult::UserNameTaken).await;
            }
        } else {
            // no user for account id - check requested name availability
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

        // ignore bypass token for now

        let bypass_token = hex::decode(
            ServerConfigService::get(AUTH_SERVICE_BYPASS_KEY.into())
                .await?
                .unwrap(),
        )
        .unwrap();

        // call auth service unless bypass token was provided and matches the configured one
        if !user_data.bypass_token.eq(&bypass_token) {
            // verify code

            // todo: move to consts
            let twilio_account_id: String = ServerConfigService::get("twilio.account_sid".into())
                .await?
                .unwrap();

            let twilio_service_id: String = ServerConfigService::get("twilio.service_id".into())
                .await?
                .unwrap();

            let twilio_token: String = ServerConfigService::get("twilio.auth_token".into())
                .await?
                .unwrap();

            let url = format!(
                "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
                serv_id = twilio_service_id,
            );

            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Content-Type",
                "application/x-www-form-urlencoded".parse().unwrap(),
            );

            let mut form_body: HashMap<&str, &String> = HashMap::new();
            form_body.insert("To", &phone_number.number);
            form_body.insert("Code", &user_data.verification_code);

            let client = Client::new();
            let res = client
                .post(url)
                .basic_auth(twilio_account_id, Some(twilio_token))
                .headers(headers)
                .form(&form_body)
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status() != StatusCode::OK {
                        info!("twilio response status code != 200");
                        return self.gen_result(VerificationResult::Unverified).await;
                    }

                    let data = response.json::<OTPVerifyResponse>().await;
                    match data {
                        Ok(result) => {
                            if result.status == "approved" {
                                // validate sid
                                if result.sid != user_data.verification_sid {
                                    info!("twilio sid mismatch");
                                    return self.gen_result(VerificationResult::MissingData).await;
                                }
                                info!("Twilio approved code!");
                            } else {
                                info!("Twilio result != approved");
                                return self.gen_result(VerificationResult::Unverified).await;
                            }
                        }
                        Err(e) => {
                            info!("error parsing twilio resp: {}", e);
                            return self.gen_result(VerificationResult::Unverified).await;
                        }
                    }
                }
                Err(e) => {
                    info!("error calling twilio: {}", e);
                    return self.gen_result(VerificationResult::Unverified).await;
                }
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
