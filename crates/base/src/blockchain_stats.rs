// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{Amount, BlockchainStats, CoinType};

impl BlockchainStats {
    pub fn new() -> Self {
        Self {
            last_block_time: chrono::Utc::now().timestamp_millis() as u64,
            tip_height: 0,
            transactions: 0,
            payments: 0,
            users: 0,
            fees: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
            signup_rewards: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
            referral_rewards: Some(Amount {
                value: 0,
                coin_type: CoinType::Core as i32
            }),
        }
    }
}

