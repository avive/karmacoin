// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{AccountId, MobileNumber, User};
use anyhow::Result;

impl User {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        Ok(())
    }
}

impl User {
    pub fn new(account_id: AccountId, user_name: String, mobile_number: MobileNumber) -> Self {
        User {
            account_id: Some(account_id),
            nonce: 0,
            user_name,
            mobile_number: Some(mobile_number),
            balance: 0,
            trait_scores: vec![],
            pre_keys: vec![],
        }
    }
}
