// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::PaymentTransactionV1;
use anyhow::{anyhow, Result};

impl PaymentTransactionV1 {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        if self.to.is_none() {
            return Err(anyhow!("mobile number is required"));
        }

        if self.amount == 0 {
            return Err(anyhow!("payment amount must be greater than 0"));
        }

        Ok(())
    }
}