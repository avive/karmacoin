// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::PaymentTransactionV1;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Display, Formatter};

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

impl Display for PaymentTransactionV1 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "PaymentTransactionV1 {{ to: {}, amount: {}, char trait id: {} }}",
            self.to.as_ref().unwrap().number,
            self.amount,
            self.char_trait_id
        )
    }
}
