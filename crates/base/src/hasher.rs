// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use sha2::{Digest, Sha256};

pub struct Hasher {}

impl Hasher {
    /*
    pub fn hash_old(data: &[u8]) -> Result<Vec<u8>> {
        let digest =
            orion::hash::digest(data).map_err(|e| anyhow!("failed to hash data: {}", e))?;

        Ok(digest.as_ref().to_vec())
    }*/

    /// Sha256 hash
    pub fn hash(data: &[u8]) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let digest = hasher.finalize();
        Ok(digest.to_vec())
    }
}
