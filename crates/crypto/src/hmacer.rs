// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use orion::hazardous::mac::hmac::sha512::*;
/// Hash based Message Auth Control implementation
pub struct Hmacer {}

impl Hmacer {
    pub fn hmac_sha512(key: &[u8], data: &[u8]) -> Result<Tag> {
        // todo: check sk is between 64 to 128 bytes
        let secret_key =
            SecretKey::from_slice(key).map_err(|_| anyhow!("invalid slice provided"))?;

        let tag =
            HmacSha512::hmac(&secret_key, data).map_err(|_| anyhow!("hmac one shot error"))?;
        Ok(tag)
    }

    pub fn hmac_sha512_verify(key: &[u8], data: &[u8], tag: Tag) -> Result<bool> {
        let secret_key =
            SecretKey::from_slice(key).map_err(|_| anyhow!("invalid slice provided"))?;

        HmacSha512::verify(&tag, &secret_key, data).map_err(|_| anyhow!("failed to verify"))?;
        Ok(true)
    }

    pub fn hmac_sha512_two_inputs(key: &[u8], input1: &[u8], input2: &[u8]) -> Result<Tag> {
        let secret_key =
            SecretKey::from_slice(key).map_err(|_| anyhow!("invalid slice provided"))?;
        let mut mac = HmacSha512::new(&secret_key);
        mac.update(input1)
            .map_err(|_| anyhow!("hmac one shot error"))?;
        mac.update(input2)
            .map_err(|_| anyhow!("hmac one shot error"))?;
        let tag = mac.finalize().map_err(|_| anyhow!("hmac one shot error"))?;
        Ok(tag)
    }
}
