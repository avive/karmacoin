// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

extern crate orion;

use self::orion::hazardous::mac::hmac::sha512::Tag;
use crate::aes_cypher::AesCypher;
use crate::hmacer::Hmacer;
use crate::kdfer::Kdfer;
use anyhow::{anyhow, Result};
use bytes::Bytes;

const HMAC_WIDTH: usize = 64; // 512 bits

/// AEAD - Authenticated Encryption with Associated Data.
pub struct AeadCipher {
    info: Bytes, // salt
    key: Bytes,  // 32 bytes key
    ad: Bytes,
}

impl AeadCipher {
    /// Initialize the cipher with the given parameters
    pub fn new(info: Bytes, key: Bytes, ad: Bytes) -> AeadCipher {
        AeadCipher { info, key, ad }
    }

    /// Encrypt a message with this cipher
    pub fn encrypt(&self, plaintext: Bytes) -> Result<Bytes> {
        // todo: look into using salt - pass salt as param.
        let salt = [0; 64];
        let mut keys = [0; 80];

        Kdfer::hkdf_sha512(&salt, &self.key.as_ref(), self.info.as_ref(), &mut keys)?;
        let encryption_key = &keys[..32];
        let authentication_key = &keys[32..64];
        let iv = &keys[64..];

        let mut cypher_text =
            AesCypher::aes256_cbc_pkcs7_encrypt(encryption_key, iv, &plaintext.as_ref())?;

        let mac = Hmacer::hmac_sha512_two_inputs(
            authentication_key,
            self.ad.as_ref(),
            cypher_text.as_ref(),
        )?;

        cypher_text.append(mac.unprotected_as_bytes().to_vec().as_mut());
        Ok(Bytes::from(cypher_text))
    }

    /// Decrypt a message with this cipher
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        // todo: use salt as param
        let msg_len = ciphertext.len();
        let enc_len = msg_len - HMAC_WIDTH;

        let salt = [0; 64];
        let mut keys = [0; 80];
        Kdfer::hkdf_sha512(&salt, &self.key.as_ref(), self.info.as_ref(), &mut keys)?;

        let encryption_key = &keys[..32];
        let authentication_key = &keys[32..64];
        let iv = &keys[64..];

        let mac = Hmacer::hmac_sha512_two_inputs(
            authentication_key,
            self.ad.as_ref(),
            &ciphertext[..enc_len],
        )?;

        let msg_mac = Tag::from_slice(&ciphertext[enc_len..])?;
        if !msg_mac.eq(&mac) {
            return Err(anyhow!("invalid message mac"));
        }

        AesCypher::aes256_cbc_pkcs7_decrypt(encryption_key, iv, &ciphertext[..enc_len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand_core::{OsRng, RngCore};

    #[test]
    fn test_encrypt_round_trip() {
        let mut info = [0u8; 32];
        OsRng.fill_bytes(&mut info);
        let info_bytes = Bytes::from(info.to_vec());

        let mut ad = [0u8; 32];
        OsRng.fill_bytes(&mut ad);
        let ad_bytes = Bytes::from(ad.to_vec());

        let mut plaintext = [0u8; 512];
        OsRng.fill_bytes(&mut plaintext);
        let plaintext_bytes = Bytes::from(plaintext.to_vec());

        let mut shared_secret = [0u8; 32];
        OsRng.fill_bytes(&mut shared_secret);
        let key = Bytes::from(shared_secret.to_vec());

        let cipher = AeadCipher::new(info_bytes, key, ad_bytes);
        let cipher_text = cipher.encrypt(plaintext_bytes).unwrap();
        let round_tripped = cipher.decrypt(&cipher_text).unwrap();
        assert_eq!(plaintext.as_ref().to_vec(), round_tripped);
    }
}
