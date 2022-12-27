// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::Signature;
use crate::karma_coin::karma_coin_verifier::VerifyNumberRequest;
use anyhow::{anyhow, Result};
use ed25519_dalek::{Keypair, Signer, Verifier};
use prost::Message;

impl VerifyNumberRequest {
    // we can't implement default here due to prost::message required derivation
    pub fn new() -> Self {
        VerifyNumberRequest {
            account_id: None,
            mobile_number: None,
            code: 0,
            nickname: "".into(),
            signature: None,
        }
    }
}

impl VerifyNumberRequest {
    pub async fn sign(&mut self, key_pair: &Keypair) -> Result<()> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        self.signature = Some(Signature {
            scheme: 0,
            signature: key_pair.sign(buf.as_slice()).as_ref().to_vec(),
        });

        Ok(())
    }

    pub fn verify_signature(&self) -> Result<()> {
        let mut cloned_req = self.clone();
        cloned_req.signature = None;
        let mut buf = Vec::with_capacity(cloned_req.encoded_len());
        if cloned_req.encode(&mut buf).is_err() {
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
