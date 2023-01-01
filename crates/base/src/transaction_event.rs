// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{
    ExecutionInfo, ExecutionResult, FeeType, SignedTransaction, TransactionEvent,
};
use chrono::Utc;

impl TransactionEvent {
    pub fn new(height: u64, tx: &SignedTransaction, transaction_hash: &[u8]) -> Self {
        Self {
            timestamp: Utc::now().timestamp_nanos() as u64,
            height,
            transaction: Some(tx.clone()),
            transaction_hash: transaction_hash.to_vec(),
            error_message: "".to_string(),
            fee_type: FeeType::Mint as i32,
            referral_reward: 0,
            signup_reward: 0,
            result: ExecutionResult::Executed as i32,
            info: ExecutionInfo::Unknown as i32,
            fee: 0,
        }
    }
}
