// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;
use ed25519_dalek::{Keypair, Signer, Verifier};
use prost::Message;
use crate::karma_coin::karma_coin_core_types::Block;

impl Block {

    /// Returns the transaction canonical hash
    pub fn get_hash(&self) -> Result<Bytes> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;
        let hash = orion::hash::digest(&buf).map_err(|e| anyhow!("failed to hash data: {}", e))?;
        Ok(Bytes::from(hash.as_ref().to_vec()))
    }

    /// Verify the signer's signature
    pub fn verify_signature(&self) -> Result<()> {

        let mut cloned = self.clone();
        cloned.signature = None;

        let mut buf = Vec::with_capacity(cloned.encoded_len());
        if cloned.encode(&mut buf).is_err() {
            return Err(anyhow!("invalid binary tx data"));
        };

        let signer = self.author.as_ref().ok_or_else(|| anyhow!("missing account id"))?;
        let signature_data = self.signature.as_ref().ok_or_else(|| anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature).map_err(|_| anyhow!("invalid signature"))
    }

    pub fn sign(
        &mut self,
        key_pair: &Keypair,
    ) -> Result<()> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::karma_coin::karma_coin_core_types::Signature;
        self.signature = Some(Signature {
            scheme: 0,
            signature: key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }
}