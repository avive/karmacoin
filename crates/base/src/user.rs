// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//


use crate::karma_coin::karma_coin_core_types::{Amount, Balance, CoinType, User};

impl User {
    /// Get current balance for provided coin type
    pub fn get_balance(&self, coin_type: CoinType) -> Amount {
        for b in self.balances.iter() {
            if let Some(amount) = b.free.as_ref() {
                if amount.coin_type == coin_type as i32 {
                    return amount.clone();
                }
            }
        }

        Amount {
            value: 0,
            coin_type: coin_type as i32,
        }
    }

    // Update balance for provided coin type with a new value
    pub fn update_balance(&mut self, new_amount: &Amount) {
        for b in self.balances.iter_mut() {
            if let Some(amount) = b.free.as_mut() {
                if amount.coin_type == new_amount.coin_type {
                    b.free = Some(new_amount.clone());
                    return;
                }
            }
        }
        self.balances.push(Balance  {
            free: Some(new_amount.clone()),
            reserved: None,
            misc_frozen: None,
            fee_frozen: None,
        })
    }

    // todo: get 12 words phrase for user private key
}
