// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::{AccountId, MobileNumber, TraitScore, User};
use anyhow::Result;
use std::fmt;
use std::fmt::{Display, Formatter};

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
            karma_score: 1, // new users have karma score of 1
        }
    }

    /// Inc the trait score for a given trait
    pub fn inc_trait_score(&mut self, trait_id: u64) {
        let mut found = false;
        for trait_score in self.trait_scores.iter_mut() {
            if trait_score.trait_id == trait_id {
                trait_score.score += 1;
                found = true;
                break;
            }
        }
        if !found {
            self.trait_scores.push(TraitScore { trait_id, score: 1 });
        }
    }
}

impl Display for User {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ user_name: {}, mobile_number: {}, account_id: {}, nonce: {}, \
            balance: {}, trait_scores: {:?}, pre_keys: {:?} }}",
            self.user_name,
            self.mobile_number.as_ref().unwrap().number,
            short_hex_string(self.account_id.as_ref().unwrap().data.as_slice()),
            self.nonce,
            self.balance,
            self.trait_scores,
            self.pre_keys
        )
    }
}
