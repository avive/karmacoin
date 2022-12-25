// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::genesis_config_service::*;
use base::karma_coin::karma_coin_core_types::BlockchainStats;

pub(crate) struct Tokenomics {
    pub(crate) stats: BlockchainStats,
}

impl Tokenomics {
    /// Get current signup reward amount based on consensus rules, genesis config and blockchain data
    pub(crate) async fn get_signup_reward_amount(&self) -> Result<u64> {
        if self.stats.users_count > GenesisConfigService::get_u64(SIGNUP_REWARD_PHASE2_ALLOCATION_KEY.into()).await?.unwrap() {
            Ok(GenesisConfigService::get_u64(SIGNUP_REWARD_PHASE3_KEY.into()).await?.unwrap())
        }
        else if self.stats.users_count > GenesisConfigService::get_u64(SIGNUP_REWARD_PHASE1_ALLOCATION_KEY.into()).await?.unwrap() {
            Ok(GenesisConfigService::get_u64(SIGNUP_REWARD_PHASE2_KEY.into()).await?.unwrap())
        }
        else {
            Ok(GenesisConfigService::get_u64(SIGNUP_REWARD_PHASE1_KEY.into()).await?.unwrap())
        }
    }

    /// Get current referral; reward amount based on consensus rules, genesis config and blockchain data
    pub(crate) async fn get_referral_reward_amount(&self) -> Result<u64> {
        if self.stats.users_count > GenesisConfigService::get_u64(REFERRAL_REWARD_PHASE2_ALLOCATION_KEY.into()).await?.unwrap() {
            Ok(0)
        } else if self.stats.users_count > GenesisConfigService::get_u64(REFERRAL_REWARD_PHASE1_ALLOCATION_KEY.into()).await?.unwrap() {
            Ok(GenesisConfigService::get_u64(REFERRAL_REWARD_PHASE2_KEY.into()).await?.unwrap())
        } else {
            Ok(GenesisConfigService::get_u64(REFERRAL_REWARD_PHASE1_KEY.into()).await?.unwrap())
        }
    }


    /// Return true iff transaction should be subsidised by the protocol
    pub(crate) async fn should_subsidise_transaction_fee(&self, user_nonce: u64, fee_amount: u64) -> Result<bool> {
        if fee_amount > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_AMOUNT.into()).await?.unwrap() {
            // tx fee too high for subsidy
            return Ok(false);
        }

        if self.stats.fee_subs_count > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_TOTAL_KEY.into()).await?.unwrap() {
            return Ok(false);
        }

        if user_nonce > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY.into()).await?.unwrap() {
            return Ok(false);
        }

        Ok(true)
    }

    /// Gets the current block reward for block producer based on block height
    pub(crate) async fn _get_block_reward_amount(&self, block_height: u64) -> Result<u64> {
        if block_height > GenesisConfigService::get_u64(BLOCK_REWARDS_LAST_BLOCK.into()).await?.unwrap() {
            // no more block rewards
            return Ok(0);
        }

        Ok(GenesisConfigService::get_u64(BLOCK_REWARDS_AMOUNT.into()).await?.unwrap())
    }
}