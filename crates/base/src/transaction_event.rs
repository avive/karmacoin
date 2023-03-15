// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::{
    ExecutionInfo, ExecutionResult, FeeType, SignedTransaction, TransactionEvent,
};
use chrono::Utc;
use std::fmt;
use std::fmt::{Display, Formatter};

impl TransactionEvent {
    pub fn new(height: u64, tx: &SignedTransaction, transaction_hash: &[u8]) -> Self {
        Self {
            timestamp: Utc::now().timestamp_millis() as u64,
            height,
            transaction: Some(tx.clone()),
            transaction_hash: transaction_hash.to_vec(),
            error_message: "".to_string(),
            fee_type: FeeType::Mint as i32,
            referral_reward: 0,
            signup_reward: 0,
            result: ExecutionResult::Executed as i32,
            info: ExecutionInfo::Unknown as i32,
            appreciation_char_trait_idx: 0,
            appreciation_community_id: 0,
            fee: 0,
        }
    }
}

impl Display for TransactionEvent {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TransactionEvent {{ timestamp: {}, height: {}, tx_hash: {}, \
        error_message: {}, fee_type: {}, referral_reward: {}, signup_reward: {}, result: {}, \
        info: {}, fee: {} }}",
            self.timestamp,
            self.height,
            short_hex_string(&self.transaction_hash),
            self.error_message,
            self.fee_type,
            self.referral_reward,
            self.signup_reward,
            self.result,
            self.info,
            self.fee // todo: add missing fields
        )
    }
}
