// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use aes::Aes256;
use anyhow::Result;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

pub struct AesCypher {}

impl AesCypher {
    pub fn aes256_cbc_pkcs7_encrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        type Aes256Cbc = Cbc<Aes256, Pkcs7>;
        let cypher = Aes256Cbc::new_from_slices(key, iv)?;
        Ok(cypher.encrypt_vec(data))
    }

    pub fn aes256_cbc_pkcs7_decrypt(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        type Aes256Cbc = Cbc<Aes256, Pkcs7>;
        let cipher = Aes256Cbc::new_from_slices(key, iv)?;
        Ok(cipher.decrypt_vec(data)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand_core::{OsRng, RngCore};

    #[test]
    fn test_encrypt_round_trip() {
        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut iv);

        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        let mut data = [0u8; 512];
        OsRng.fill_bytes(&mut data);

        let cipher_text =
            AesCypher::aes256_cbc_pkcs7_encrypt(key.as_ref(), iv.as_ref(), data.as_ref()).unwrap();

        let round_tripped_data =
            AesCypher::aes256_cbc_pkcs7_decrypt(key.as_ref(), iv.as_ref(), cipher_text.as_ref())
                .unwrap();

        assert_eq!(data.to_vec().as_slice(), round_tripped_data.as_slice());
    }
}
