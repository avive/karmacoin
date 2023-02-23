// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::Block;
use crate::signed_trait::SignedTrait;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use ed25519_dalek::{PublicKey, Signature};
use orion::hazardous::hash::sha2::sha256::Sha256;
use prost::Message;

impl SignedTrait for Block {
    fn get_sign_message(&self) -> Result<Vec<u8>> {
        let mut cloned = self.clone();
        cloned.signature = None;
        let mut buf = Vec::with_capacity(cloned.encoded_len());
        cloned.encode(&mut buf)?;
        Ok(buf.to_vec())
    }

    fn get_signature(&self) -> Result<Signature> {
        Ok(Signature::from_bytes(
            &self
                .signature
                .as_ref()
                .ok_or_else(|| anyhow!("no signature found"))?
                .signature
                .clone(),
        )?)
    }

    fn get_public_key(&self) -> Result<PublicKey> {
        Ok(PublicKey::from_bytes(
            self.author
                .as_ref()
                .ok_or_else(|| anyhow!("missing key data"))?
                .data
                .as_slice(),
        )?)
    }
}

impl Block {
    /// Returns the transaction canonical hash
    pub fn get_hash(&self) -> Result<Bytes> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let digest = Sha256::digest(buf.as_ref())?;
        Ok(Bytes::from(digest.as_ref().to_vec()))
    }
}
