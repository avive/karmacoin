// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verifier_service::VerifierService;
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_verifier::SendVerificationCodeRequest;
use base::server_config_service::ServerConfigService;
use http::{header, StatusCode};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use xactor::*;

#[message(result = "Result<String>")]
pub(crate) struct SendVerificationCode(pub SendVerificationCodeRequest);

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyRequest {
    pub sid: String,
}
/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<SendVerificationCode> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SendVerificationCode,
    ) -> Result<String> {
        let req = msg.0;

        info!("sending verification code to {}", req.mobile_number);

        if req.mobile_number.is_empty() {
            return Err(anyhow!("Missing mobile number"));
        }

        if !req.mobile_number.starts_with("+") {
            return Err(anyhow!("Invalid mobile number. Should start with +"));
        }

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
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = twilio_service_id,
        );

        let whatsapp: String = "whatsapp".to_string();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();
        form_body.insert("To", &req.mobile_number);
        form_body.insert("Channel", &whatsapp);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(twilio_account_id, Some(twilio_token))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        return match res {
            Ok(response) => {
                if response.status() != StatusCode::CREATED {
                    info!("twilio response status code != 201");
                    return Err(anyhow!("Bad Twilio api response"));
                }

                let data = response.json::<OTPVerifyRequest>().await;
                match data {
                    Ok(result) => {
                        info!("Send verification code via whatsapp");
                        Ok(result.sid)
                    }
                    Err(e) => {
                        info!("error parsing twilio resp: {}", e);
                        Err(anyhow!("Bad Twilio api response"))
                    }
                }
            }
            Err(e) => {
                info!("error calling twilio: {}", e);
                Err(anyhow!("Can't call Twilio"))
            }
        };
    }
}
