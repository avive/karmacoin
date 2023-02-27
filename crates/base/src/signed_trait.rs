// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hasher::Hasher;
use crate::hex_utils::hex_string;
use crate::karma_coin::karma_coin_core_types::Signature;
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signer;
use ed25519_dalek::{Keypair, Verifier};
use log::*;

pub trait SignedTrait {
    /// return the data of the message that is signed by this type
    fn get_sign_message(&self) -> Result<Vec<u8>>;

    /// return signature provided by this type
    fn get_signature(&self) -> Result<ed25519_dalek::Signature>;

    /// return the public key provided by this type
    fn get_public_key(&self) -> Result<ed25519_dalek::PublicKey>;

    /// Verify the signature of this type
    fn verify_signature(&self) -> Result<()> {
        use ed25519_dalek::ed25519::signature::Signature;

        let signature = &self.get_signature()?;
        let message = &self.get_sign_message()?;
        let pub_key = &self.get_public_key().unwrap();

        info!("Debug info:");

        // message hash
        let hash = Hasher::hash(message)?;

        info!("Message hash: {}", hex_string(&hash));
        info!("Signature: {}", hex_string(signature.as_bytes()));
        info!("Pub key: {}", hex_string(pub_key.to_bytes().as_ref()));

        self.get_public_key()?
            .verify(&self.get_sign_message()?, &self.get_signature()?)
            .map_err(|e| anyhow!("invalid signature: {:?}", e))
    }

    /// Sign the message of this type. Note that signature will not set on the type
    /// and this needs to be explicitly done by the caller
    fn sign(&self, key_pair: &Keypair) -> Result<Signature> {
        let data = self.get_sign_message()?;

        Ok(Signature {
            scheme: 0,
            signature: key_pair.sign(data.as_slice()).as_ref().to_vec(),
        })
    }
}
