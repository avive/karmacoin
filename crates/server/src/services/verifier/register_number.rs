// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, VERIFICATION_CODES_COL_FAMILY,
};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_verifier::{
    RegisterNumberRequest, RegisterNumberResponse, RegisterNumberResult::*,
};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use xactor::*;

use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::AccountId;
use base::signed_trait::SignedTrait;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[message(result = "Result<RegisterNumberResponse>")]
pub(crate) struct RegisterNumber(pub RegisterNumberRequest);

/// Request to register a phone number for an account
#[async_trait::async_trait]
impl Handler<RegisterNumber> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RegisterNumber,
    ) -> Result<RegisterNumberResponse> {
        let verifier_key_pair = self.get_key_pair().await?.to_ed2559_keypair();
        info!(
            "Verifier account id: {:?}",
            short_hex_string(&verifier_key_pair.public.to_bytes())
        );

        let req = msg.0;
        req.verify_signature()?;

        let account_id = req
            .account_id
            .ok_or_else(|| anyhow!("missing account id"))?;
        let phone_number = req
            .mobile_number
            .ok_or_else(|| anyhow!("missing mobile phone number"))?;

        // check if number is already registered to another user
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(phone_number.number.as_bytes().to_vec()),
            cf: MOBILE_NUMBERS_COL_FAMILY,
        })
        .await?
        {
            return if user_data.0 == account_id.data {
                // number already registered for this account
                let mut resp = RegisterNumberResponse::from(NumberAlreadyRegistered);
                resp.account_id = Some(AccountId {
                    data: verifier_key_pair.public.as_bytes().to_vec(),
                });

                resp.signature = Some(resp.sign(&verifier_key_pair)?);
                Ok(resp)
            } else {
                // number already registered for another account
                let mut resp = RegisterNumberResponse::from(NumberAccountExists);
                resp.account_id = Some(AccountId {
                    data: verifier_key_pair.public.as_bytes().to_vec(),
                });
                resp.signature = Some(resp.sign(&verifier_key_pair)?);
                Ok(resp)
            };
        }

        // todo: send new verification code via sms to user

        // generate a random 6 digits code
        let code = ChaCha20Rng::from_entropy().gen_range(100_000..999_999);
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, code as u32);

        info!(
            "Sent verification code {:?}, to accountID: {:?}",
            code,
            short_hex_string(&account_id.data)
        );

        // store verificationCode -> accountNumber with ttl of 24 hours in

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(buf.to_vec()),
                value: Bytes::from(account_id.data.to_vec()),
            },
            cf: VERIFICATION_CODES_COL_FAMILY,
            ttl: 60 * 60 * 24, // 24 hours ttl
        })
        .await?;

        let mut resp = RegisterNumberResponse::from(CodeSent);
        resp.code = code;
        // resp.signature = Some(resp.sign(&verifier_key_pair)?);
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::services::db_config_service::DbConfigService;
    use crate::services::verifier::register_number::RegisterNumber;
    use crate::services::verifier::verifier_service::VerifierService;
    use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair, MobileNumber};
    use base::karma_coin::karma_coin_verifier::{
        RegisterNumberRequest, RegisterNumberResponse, RegisterNumberResult::*,
    };
    use base::signed_trait::SignedTrait;
    use base::test_helpers::enable_logger;
    use db::db_service::DatabaseService;
    use xactor::Service;

    #[tokio::test(flavor = "multi_thread")]
    async fn register_number_test() {
        // init logging
        enable_logger();

        // config the db
        DbConfigService::from_registry().await.unwrap();

        // do the test here...

        let client_key_pair = KeyPair::new();
        let client_ed_key_pair = client_key_pair.to_ed2559_keypair();

        let mut register_number_request = RegisterNumberRequest::new();
        register_number_request.mobile_number = Some(MobileNumber {
            number: "972549805380".to_string(),
        });
        let account_id = client_ed_key_pair.public.to_bytes().to_vec();
        register_number_request.account_id = Some(AccountId { data: account_id });
        register_number_request.signature =
            Some(register_number_request.sign(&client_ed_key_pair).unwrap());

        let verifier = VerifierService::from_registry().await.unwrap();

        let req = RegisterNumber(register_number_request);
        let resp: RegisterNumberResponse = verifier.call(req).await.unwrap().unwrap();
        assert_eq!(resp.result, CodeSent as i32);

        // drop the db
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        db_service.stop(None).expect("failed to stop the db");
    }
}
