// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ByteOrder};
use bytes::Bytes;
use ed25519_dalek::{Verifier};
use base::karma_coin::karma_coin_verifier::VerifyNumberRequest;
use base::karma_coin::karma_coin_core_types::{VerifyNumberResponse, VerifyNumberResult::*};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, NICKS_COL_FAMILY, RESERVED_NICKS_COL_FAMILY, VERIFICATION_CODES_COL_FAMILY};
use crate::services::verifier::verifier_service::VerifierService;

#[message(result = "Result<VerifyNumberResponse>")]
pub(crate) struct Verify(pub VerifyNumberRequest);

// Request to sign up
#[async_trait::async_trait]
impl Handler<Verify> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: Verify,
    ) -> Result<VerifyNumberResponse> {

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
        let nickname = req.nickname;
        let signature_data = req.signature.ok_or(anyhow!("missing signature"))?;

        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature)?;

        // decode auth code number
        let auth_code : u32 = req.code.parse::<u32>().map_err(|_| anyhow!("invalid auth code"))?;

        let verifier_key_pair = self.id_key_pair.as_ref().unwrap().to_ed2559_kaypair();

        // db key based on auth code
        let mut auth_code_buf = [0; 4];
        LittleEndian::write_u32(&mut auth_code_buf, auth_code);

        let auth_data = DatabaseService::read(ReadItem {
            key: Bytes::from(auth_code_buf.to_vec()),
            cf: VERIFICATION_CODES_COL_FAMILY
        }).await?;

        if auth_data.is_none() {
            let mut resp = VerifyNumberResponse::from(InvalidCode);
            resp.sign(&verifier_key_pair)?;
            return Ok(resp);
        }

        // check that code was sent to the caller's account id
        let sent_account_id = auth_data.unwrap().0.to_vec();
        if account_id.data != sent_account_id {
            // code was sent to a different account
            let mut resp = VerifyNumberResponse::from(InvalidCode);
            resp.sign(&verifier_key_pair)?;
            return Ok(resp);
        }

        // check that no other account was created with this mobile number

        let phone_number = req.mobile_number.ok_or_else(|| anyhow!("missing mobile phone number"))?;

        if (DatabaseService::read(ReadItem {
            key: Bytes::from(phone_number.number.as_bytes().to_vec()),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await?).is_some() {
            let mut resp = VerifyNumberResponse::from(NumberAlreadyRegisteredOtherAccount);
            resp.sign(&verifier_key_pair)?;
            return Ok(resp);
        }

        // verify that the requested nickname not registered to another user
        let nick_name_key = Bytes::from(nickname.as_bytes().to_vec());

        if (DatabaseService::read(ReadItem {
            key: nick_name_key.clone(),
            cf: NICKS_COL_FAMILY
        }).await?).is_some() {
            let mut resp = VerifyNumberResponse::from(NicknameTaken);
            resp.sign(&verifier_key_pair)?;
            return Ok(resp);
        }

        // verify the the requested nickname is not reserved by a new user over the last 24 hours
        if (DatabaseService::read(ReadItem {
            key: nick_name_key.clone(),
            cf: RESERVED_NICKS_COL_FAMILY
        }).await?).is_some() {
            let mut resp = VerifyNumberResponse::from(NicknameTaken);
            resp.sign(&verifier_key_pair)?;
            return Ok(resp);
        }

        // reserve the nickname for the caller account for 24 hours
        DatabaseService::write(WriteItem {
            data: DataItem { key: nick_name_key, value: Bytes::from(account_id.data.to_vec()) },
            cf: RESERVED_NICKS_COL_FAMILY,
            ttl: 60 * 60 * 24
        }).await?;

        // create signed Response and return it
        let mut resp = VerifyNumberResponse::from(Verified);

        // signed attestation details - user account id, nickname and verified mobile number
        resp.account_id = Some(account_id);
        resp.nickname = nickname;
        resp.mobile_number = Some(phone_number);
        resp.sign(&self.id_key_pair.as_ref().unwrap().to_ed2559_kaypair())?;
        Ok(resp)

    }
}