use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ByteOrder};
use bytes::Bytes;
use ed25519_dalek::{Verifier};
use rand_chacha::ChaCha20Rng;
use rand::prelude::*;
use rand_core::SeedableRng;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, RegisterNumberResponse, RegisterNumberResult};
use base::karma_coin::karma_coin_core_types::{VerifyNumberResult::*};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, VERIFICATION_CODES_COL_FAMILY};
use crate::services::verifier_service::VerifierService;

#[message(result = "Result<RegisterNumberResponse>")]
pub(crate) struct RegisterNumber(pub RegisterNumberRequest);

// Request to register a phone number for an account
#[async_trait::async_trait]
impl Handler<RegisterNumber> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RegisterNumber,
    ) -> Result<RegisterNumberResponse> {

        let req = msg.0;

        // Verify signature
        let mut cloned_req = req.clone();
        cloned_req.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(cloned_req.encoded_len());
        if cloned_req.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let account_id = req.account_id.ok_or(anyhow!("missing account id"))?;
        let signature_data = req.signature.ok_or(anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature)?;

        let phone_number = req.mobile_number.ok_or(anyhow!("missing mobile phone number"))?;

        // check signature by accountId private key on data so we know caller has private key for accountId

        // check if number is already registered to another user
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(bincode::serialize(&phone_number.number).unwrap()),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await? {
            // number already registered for a user account

            // compare account ids
            return if user_data.0 == account_id.data {
                Ok(RegisterNumberResponse {
                    result: NumberAlreadyRegisteredThisAccount as i32
                })
            } else {
                Ok(RegisterNumberResponse {
                    result: NumberAlreadyRegisteredOtherAccount as i32
                })
            }
        }

        // todo: send new verification code via sms to user

        // generate a random 6 digits code
        let code = ChaCha20Rng::from_entropy().gen_range(100_000..999_999);
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, code as u32);

        info!("Sent verification code {:?}, to accountID: {:?}", code, account_id.data);

        // store verificationCode -> accountNumber with ttl of 24 hours in

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(buf.to_vec()),
                value: Bytes::from(account_id.data.to_vec()) },
            cf: VERIFICATION_CODES_COL_FAMILY,
            ttl: 60 * 60 * 24, // 24 hours ttl
        }).await?;


        Ok(RegisterNumberResponse {
            result: RegisterNumberResult::CodeSent as i32
        })

    }
}
