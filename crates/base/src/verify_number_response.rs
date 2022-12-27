// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{VerifyNumberResponse, VerifyNumberResult};
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use ed25519_dalek::{Keypair, Signer, Verifier};

impl VerifyNumberResponse {
    // we can't implement default here due to prost::message required derivation
    fn new() -> Self {
        VerifyNumberResponse {
            timestamp: Utc::now().timestamp_nanos() as u64,
            result: 0,
            nickname: "".into(),
            account_id: None,
            mobile_number: None,
            signature: None,
        }
    }
}

// todo: add validate function

impl From<VerifyNumberResult> for VerifyNumberResponse {
    fn from(result: VerifyNumberResult) -> Self {
        let mut resp = VerifyNumberResponse::new();
        resp.result = result as i32;
        resp
    }
}

impl VerifyNumberResponse {
    pub fn sign(&mut self, key_pair: &Keypair) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::karma_coin::karma_coin_core_types::Signature;
        self.signature = Some(Signature {
            scheme: 0,
            signature: key_pair.sign(&buf).as_ref().to_vec(),
        });

        let mut buf1 = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf1)?;

        self.signature = Some(Signature {
            scheme: 0,
            signature: key_pair.sign(&buf1).as_ref().to_vec(),
        });
        Ok(())
    }

    pub fn verify_signature(&self) -> Result<()> {
        let mut cloned = self.clone();
        cloned.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(cloned.encoded_len());
        if cloned.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };
        let account_id = self
            .account_id
            .as_ref()
            .ok_or(anyhow!("missing account id"))?;
        let signature_data = self
            .signature
            .as_ref()
            .ok_or(anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_slice())?;
        signer_pub_key
            .verify(&buf, &signature)
            .map_err(|_| anyhow!("failed to verify signature"))
    }
}
