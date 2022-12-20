// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};

// Hash uses BLAKE2b with an output size of 32 bytes (i.e BLAKE2b-256).
pub struct Hash {}

impl Hash {
    pub fn hash(data: &[u8]) -> Result<Vec<u8>> {
        let digest =
            orion::hash::digest(data).map_err(|e| anyhow!("failed to hash data: {}", e))?;

        Ok(digest.as_ref().to_vec())
    }
}
