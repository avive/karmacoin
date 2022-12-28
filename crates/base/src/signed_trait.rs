// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::Signature;
use anyhow::{anyhow, Result};
use ed25519_dalek::ed25519::signature::Signer;
use ed25519_dalek::{Keypair, Verifier};

pub trait SignedTrait {
    /// return the data of the message that is signed by this type
    fn get_sign_message(&self) -> Result<Vec<u8>>;

    /// return signature provided by this type
    fn get_signature(&self) -> Result<ed25519_dalek::Signature>;

    /// return the public key provided by this type
    fn get_public_key(&self) -> Result<ed25519_dalek::PublicKey>;

    /// Verify the signature of this type
    fn verify_signature(&self) -> Result<()> {
        self.get_public_key()?
            .verify(&self.get_sign_message()?, &self.get_signature()?)
            .map_err(|_| anyhow!("failed to verify signature"))
    }

    /// Sign the message of this type
    fn sign(&self, key_pair: &Keypair) -> Result<Signature> {
        let data = self.get_sign_message()?;

        Ok(Signature {
            scheme: 0,
            signature: key_pair.sign(data.as_slice()).as_ref().to_vec(),
        })
    }
}
