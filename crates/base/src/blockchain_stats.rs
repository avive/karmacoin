// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::BlockchainStats;

impl BlockchainStats {
    pub fn new() -> Self {
        Self {
            last_block_time: 0,
            tip_height: 0,
            transactions_count: 0,
            payments_transactions_count: 0,
            update_user_transactions_count: 0,
            users_count: 0,
            fees_amount: 0,
            minted_amount: 0,
            circulation: 0,
            fee_subs_count: 0,
            fee_subs_amount: 0,
            signup_rewards_amount: 0,
            signup_rewards_count: 0,
            referral_rewards_amount: 0,
            validator_rewards_count: 0,
            referral_rewards_count: 0,
            validator_rewards_amount: 0,
            // todo: get this from external data file
            exchange_rate: 0.2,
            causes_rewards_amount: 0,
            appreciations_transactions_count: 0,
        }
    }
}
