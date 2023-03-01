// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::PaymentTransactionV1;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Display, Formatter};

impl PaymentTransactionV1 {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        if self.from.is_none() {
            return Err(anyhow!("sender's account id is required"));
        }

        if self.to_number.is_none() && self.to_account_id.is_none() {
            return Err(anyhow!(
                "payee mobile number OR payee account id is required"
            ));
        }

        if self.amount == 0 {
            return Err(anyhow!("payment amount must be greater than 0"));
        }

        Ok(())
    }
}

impl Display for PaymentTransactionV1 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.to_number {
            Some(ref to_number) => {
                write!(
                    f,
                    "PaymentTransactionV1 {{ from: {}, to: {}, amount: {}, char trait id: {} }}",
                    short_hex_string(self.from.as_ref().unwrap().data.as_ref()),
                    to_number.number,
                    self.amount,
                    self.char_trait_id
                )
            }
            None => {
                write!(
                    f,
                    "PaymentTransactionV1 {{ from: {}, to: {}, amount: {}, char trait id: {} }}",
                    short_hex_string(self.from.as_ref().unwrap().data.as_ref()),
                    short_hex_string(self.to_account_id.as_ref().unwrap().data.as_ref()),
                    self.amount,
                    self.char_trait_id
                )
            }
        }
    }
}
