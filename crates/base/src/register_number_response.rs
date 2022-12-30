// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_verifier::{RegisterNumberResponse, RegisterNumberResult};
use crate::signed_trait::SignedTrait;
use anyhow::{anyhow, Result};
use prost::Message;

impl SignedTrait for RegisterNumberResponse {
    fn get_sign_message(&self) -> Result<Vec<u8>> {
        let mut cloned = self.clone();
        cloned.signature = None;
        let mut buf = Vec::with_capacity(cloned.encoded_len());
        cloned.encode(&mut buf)?;
        Ok(buf.to_vec())
    }

    fn get_signature(&self) -> Result<ed25519_dalek::Signature> {
        Ok(ed25519_dalek::Signature::from_bytes(
            &self
                .signature
                .as_ref()
                .ok_or_else(|| anyhow!("no signature found"))?
                .signature
                .clone(),
        )?)
    }

    fn get_public_key(&self) -> Result<ed25519_dalek::PublicKey> {
        Ok(ed25519_dalek::PublicKey::from_bytes(
            &self
                .account_id
                .as_ref()
                .ok_or_else(|| anyhow!("no public found"))?
                .data,
        )?)
    }
}

impl RegisterNumberResponse {
    // we can't implement default here due to prost::message required derivation
    pub fn new() -> Self {
        RegisterNumberResponse {
            result: 0,
            code: 0,
            signature: None,
            account_id: None,
        }
    }
}

impl From<RegisterNumberResult> for RegisterNumberResponse {
    fn from(result: RegisterNumberResult) -> Self {
        let mut resp = RegisterNumberResponse::new();
        resp.result = result as i32;
        resp
    }
}
