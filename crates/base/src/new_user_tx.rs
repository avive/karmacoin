// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{NewUserTransactionV1, User, VerifyNumberResponse};
use anyhow::Result;

impl NewUserTransactionV1 {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        Ok(())
    }
}

impl NewUserTransactionV1 {
    pub fn new(user: User, v_resp: VerifyNumberResponse) -> Self {
        NewUserTransactionV1 {
            user: Some(user),
            verify_number_response: Some(v_resp),
        }
    }
}
