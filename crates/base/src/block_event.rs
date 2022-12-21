// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{Amount, BlockEvent, CoinType, TransactionEvent};

impl BlockEvent {
    pub fn new(height: u64) -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            height,
            block_hash: vec![],
            total_signups: 0,
            total_payments: 0,
            total_fees: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
            total_signup_rewards: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
            total_referral_rewards: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
            transactions_events: vec![]
        }
    }

    pub fn add_signup_reward(&mut self, value: u64) {
        self.total_signup_rewards.as_mut().unwrap().value += value;
    }

    pub fn add_referral_reward(&mut self, value: u64) {
        self.total_referral_rewards.as_mut().unwrap().value += value;
    }

    pub fn add_fee(&mut self, value: u64) {
        self.total_fees.as_mut().unwrap().value += value;
    }

    pub fn inc_total_payments(&mut self) {
        self.total_payments += 1;
    }

    pub fn inc_total_signups(&mut self) {
        self.total_signups += 1;
    }

    pub fn add_transaction_event(&mut self, event: TransactionEvent) {
        self.transactions_events.push(event);
    }
}

