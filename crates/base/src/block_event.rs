// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{BlockEvent, TransactionEvent};

impl BlockEvent {
    pub fn new(height: u64) -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            height,
            block_hash: vec![],
            signups_count: 0,
            payments_count: 0,
            fees_amount: 0,
            signup_rewards_amount: 0,
            referral_rewards_amount: 0,
            transactions_events: vec![],
            reward: 0,
            referral_rewards_count: 0,
        }
    }

    pub fn add_signup_reward(&mut self, value: u64) {
        self.signup_rewards_amount += value;
    }

    pub fn add_referral_reward(&mut self, value: u64) {
        self.referral_rewards_amount += value;
    }

    pub fn add_fee(&mut self, value: u64) {
        self.fees_amount += value;
    }

    pub fn inc_total_payments(&mut self) {
        self.payments_count += 1;
    }

    pub fn inc_total_signups(&mut self) {
        self.signups_count += 1;
    }

    pub fn add_transaction_event(&mut self, event: TransactionEvent) {
        self.transactions_events.push(event);
    }
}
