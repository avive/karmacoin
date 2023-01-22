// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::UpdateUserTransactionV1;
use anyhow::{anyhow, Result};

impl UpdateUserTransactionV1 {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        if self.nickname.is_empty()
            && (self.user_verification_data.is_none() || self.mobile_number.is_none())
        {
            return Err(anyhow!(
                "expected non-empty requested nickname or verify number response and mobile number"
            ));
        }

        Ok(())
    }
}
