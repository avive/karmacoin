// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::hex_string;
use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::Signature as KarmaCoinSignature;
use crate::karma_coin::karma_coin_core_types::{SignedTransaction, TransactionBody};
use anyhow::{anyhow, Result};
use bytes::Bytes;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use log::info;
use orion::hazardous::hash::sha2::sha256::Sha256;
use prost::Message;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for SignedTransaction {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "SignedTransaction {{ signer: {}, hash: {}, signature: {} }}",
            short_hex_string(&self.signer.as_ref().unwrap().data),
            short_hex_string(self.get_hash().unwrap().as_ref()),
            short_hex_string(self.get_signature().unwrap().as_ref()),
        )
    }
}

impl SignedTransaction {
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
            self.signer
                .as_ref()
                .ok_or_else(|| anyhow!("missing key data"))?
                .data
                .as_slice(),
        )?)
    }

    pub fn sign(&self, key_pair: &Keypair) -> Result<KarmaCoinSignature> {
        Ok(KarmaCoinSignature {
            scheme: 0,
            signature: key_pair
                .sign(self.transaction_body.as_slice())
                .as_ref()
                .to_vec(),
        })
    }

    /// Verify the signature of this type
    fn verify_signature(&self) -> Result<()> {
        use ed25519_dalek::ed25519::signature::Signature;

        let signature = &self.get_signature()?;
        let pub_key = &self.get_public_key().unwrap();

        info!("Debug info:");

        // message hash
        let digest = Sha256::digest(self.transaction_body.as_slice())?;
        let hash = digest.as_ref().to_vec();

        info!("Message hash: {}", hex_string(&hash));
        info!("Signature: {}", hex_string(signature.as_bytes()));
        info!("Pub key: {}", hex_string(pub_key.to_bytes().as_ref()));

        self.get_public_key()?
            .verify(self.transaction_body.as_ref(), &self.get_signature()?)
            .map_err(|e| anyhow!("invalid signature: {:?}", e))
    }

    /// Returns the transaction canonical hash
    pub fn get_hash(&self) -> Result<Bytes> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        // todo: refactor it to a canonical hash function for the project
        let digest = Sha256::digest(buf.as_ref())?;
        Ok(Bytes::from(digest.as_ref().to_vec()))
    }

    /// Validate transaction has valid syntax, fields has the correct net id and is properly
    /// signed before processing it
    pub async fn validate(&self) -> Result<()> {
        self.verify_syntax().await?;
        self.verify_signature()
    }

    pub async fn verify_syntax(&self) -> Result<()> {
        if self.signer.is_none() {
            return Err(anyhow!("required signer is missing"));
        }

        if self.signature.is_none() {
            return Err(anyhow!("required signature is missing"));
        }

        if self.transaction_body.len() == 0 {
            return Err(anyhow!("required transaction body is missing"));
        }

        Ok(())
    }

    pub fn get_body(&self) -> Result<TransactionBody> {
        Ok(TransactionBody::decode(self.transaction_body.as_ref())?)
    }
}
