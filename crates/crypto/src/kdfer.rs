// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};

use bytebuffer::ByteBuffer;
use orion::hazardous::kdf::hkdf;

const SALT: &str = "upsetter secure messaging experiment";

/// Kdfer provides a kdf service. Implemented as a thin wrapper over orion.
/// As functions are completely stateless - no synchronization needed
pub struct Kdfer {}

impl Kdfer {
    // Derive a 32 bytes key from 2 to 4 key materials
    pub fn kdf(dh1: &[u8], dh2: &[u8], dh3: Option<&[u8]>, dh4: Option<&[u8]>) -> Result<[u8; 32]> {
        let mut buf = ByteBuffer::new();
        let prefix: Vec<u8> = std::iter::repeat(0xFF).take(32).collect();
        buf.write_bytes(&prefix);
        buf.write_bytes(dh1);
        buf.write_bytes(dh2);

        if let Some(val) = dh3 {
            buf.write_bytes(val);
        }

        if let Some(val) = dh4 {
            buf.write_bytes(val);
        }

        let mut res = [0u8; 32];
        hkdf::sha512::derive_key(SALT.as_bytes(), buf.to_bytes().as_slice(), None, &mut res)
            .map_err(|e| anyhow!("hkdef derive key crypto failed: {}", e))?;

        Ok(res)
    }

    // Derive a 64 bytes key from input, salt and optional info
    pub fn hkdf(salt: &[u8], input: &[u8], info: &[u8]) -> Result<[u8; 64]> {
        let mut key = [0; 64];

        hkdf::sha512::derive_key(&salt, &input, Some(info), &mut key)
            .map_err(|e| anyhow!("hkdef derive failure: {}", e))?;

        Ok(key)
    }

    pub fn hkdf_sha512(salt: &[u8], input: &[u8], info: &[u8], key: &mut [u8]) -> Result<()> {
        hkdf::sha512::derive_key(salt, input, Some(info), key)
            .map_err(|e| anyhow!("hkdef derive key failed: {}", e))?;

        Ok(())
    }
}
