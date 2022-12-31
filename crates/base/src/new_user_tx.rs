// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::NewUserTransactionV1;
use anyhow::Result;

impl NewUserTransactionV1 {
    /// todo: verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        Ok(())
    }
}
