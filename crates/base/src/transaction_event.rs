// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use chrono::Utc;
use crate::karma_coin::karma_coin_core_types::{ExecutionResult, FeeType, SignedTransaction, TransactionEvent};

impl TransactionEvent {
    pub fn new(height: u64, tx: &SignedTransaction, transaction_hash: &[u8]) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            height,
            transaction: Some(tx.clone()),
            transaction_hash: transaction_hash.to_vec(),
            result: ExecutionResult::Executed as i32,
            error_message: "".to_string(),
            fee_type: FeeType::Mint as i32,
            referral_reward: 0,
            signup_reward: 0,
        }
    }
}

