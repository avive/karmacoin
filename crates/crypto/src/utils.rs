// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use curve25519_dalek::edwards::CompressedEdwardsY;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha512};
use std::convert::TryFrom;
use x25519_dalek::StaticSecret;

const ADDRESS_LEN: usize = 20; // bytes

/// Returns a random number between 0 and max_value exclusive
pub fn get_random(max_value: u32) -> u32 {
    let mut rng = OsRng;
    rng.next_u32() % max_value
}

/// Returns sha512 of the data
pub fn sha512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Converts from an ed25519 public key to an x25519 public key
pub struct PublicKeyWrapper(pub x25519_dalek::PublicKey);
impl From<PublicKey> for PublicKeyWrapper {
    // todo: change to TryFrom because decompress() can return an error
    fn from(key: PublicKey) -> Self {
        let ed25519_pk_c = CompressedEdwardsY::from_slice(key.as_bytes());
        let ed25519_pk = ed25519_pk_c.decompress().unwrap();
        let pub_key = x25519_dalek::PublicKey::from(ed25519_pk.to_montgomery().to_bytes());
        PublicKeyWrapper(pub_key)
    }
}

/// Converts from bytes array to x25519::StaticSecret
pub struct StaticSecretWrapper(pub StaticSecret);
impl TryFrom<&[u8]> for StaticSecretWrapper {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(anyhow!("invalid slice size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(StaticSecretWrapper(StaticSecret::from(bytes)))
    }
}

pub struct X25519PublicKeyWrapper(pub x25519_dalek::PublicKey);

/// Converts from bytes to an x25519::PublicKey
impl TryFrom<&[u8]> for X25519PublicKeyWrapper {
    type Error = anyhow::Error;

    fn try_from(slice: &[u8]) -> Result<Self> {
        if slice.len() != 32 {
            return Err(anyhow!("invalid slice size != 32"));
        }

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(slice);
        Ok(X25519PublicKeyWrapper(x25519_dalek::PublicKey::from(bytes)))
    }
}

/// Convert an ed25519 secret key to an x25519 static secret
impl From<&SecretKey> for StaticSecretWrapper {
    fn from(key: &SecretKey) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(key.as_bytes().to_vec());
        let hash = hasher.finalize();
        let mut data = [0; 32];
        for i in 0..32 {
            data[i] = hash[i];
        }
        StaticSecretWrapper(StaticSecret::from(data))
    }
}

/// Generate a pair of ed25519 identity keys
#[allow(dead_code)]
pub fn create_key_pair() -> Keypair {
    Keypair::generate(&mut rand_core::OsRng)
}

/// Create an immutable Address from a public key
#[allow(dead_code)]
pub fn create_address(key: &[u8]) -> Vec<u8> {
    key[(PUBLIC_KEY_LENGTH - ADDRESS_LEN)..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{create_address, create_key_pair};
    use base::tests_helpers::enable_logger;

    #[test]
    fn test_utils() {
        enable_logger();

        let keys = create_key_pair();

        debug!(
            "Pub key: {:?}, Private kay: {:?}",
            hex::encode(keys.public),
            hex::encode(keys.secret),
        );
        let address = create_address(keys.public.as_ref());
        debug!("Address {:?}", hex::encode(address.to_vec()));

        assert_eq!(address.len(), ADDRESS_LEN, "expected non-empty address");
        assert_eq!(
            address,
            &keys.public.as_bytes()[(PUBLIC_KEY_LENGTH - ADDRESS_LEN)..]
        );
    }
}
