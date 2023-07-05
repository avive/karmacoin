// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::genesis_config_service::*;
use base::karma_coin::karma_coin_core_types::{BlockchainStats, TransactionType};

pub struct Tokenomics {
    pub stats: BlockchainStats,
}

impl Tokenomics {
    pub fn new(stats: BlockchainStats) -> Self {
        Self { stats }
    }
}

impl Tokenomics {
    pub async fn get_karma_coin_reward_amount(&self) -> Result<u64> {
        let karma_rewards_allocation =
            GenesisConfigService::get_u64(KARAM_REWARDS_ALLOCATION_KEY.into())
                .await?
                .unwrap()
                * ONE_KC_IN_KCENTS;

        if self.stats.karma_rewards_amount >= karma_rewards_allocation {
            return Ok(0);
        }

        Ok(GenesisConfigService::get_u64(KARMA_REWARD_AMOUNT.into())
            .await?
            .unwrap())
    }

    /// Get current signup reward amount based on consensus rules, genesis config and blockchain data
    pub async fn get_signup_reward_amount(&self) -> Result<u64> {
        Ok(
            GenesisConfigService::get_u64(SIGNUP_REWARD_AMOUNT_PHASE1_KEY.into())
                .await?
                .unwrap(),
        )
        /*
        // In KCents
        let signup_rewards_alloc_phase1 =
            GenesisConfigService::get_u64(SIGNUP_REWARD_ALLOCATION_PHASE1_KEY.into())
                .await?
                .unwrap()
                * ONE_KC_IN_KCENTS;

        // In KCents
        let signup_rewards_alloc_phase2 =
            GenesisConfigService::get_u64(SIGNUP_REWARD_ALLOCATION_PHASE1_KEY.into())
                .await?
                .unwrap()
                * ONE_KC_IN_KCENTS;

        if self.stats.signup_rewards_amount
            > signup_rewards_alloc_phase1 + signup_rewards_alloc_phase2
        {
            // We are in phase 3
            Ok(
                GenesisConfigService::get_u64(SIGNUP_REWARD_AMOUNT_PHASE3_KEY.into())
                    .await?
                    .unwrap(),
            )
        } else if self.stats.signup_rewards_amount > signup_rewards_alloc_phase1 {
            // We are in phase 2
            Ok(
                GenesisConfigService::get_u64(SIGNUP_REWARD_AMOUNT_PHASE2_KEY.into())
                    .await?
                    .unwrap(),
            )
        } else {
            info!("Signup rewards phase I");
            // We are in phase 1
            Ok(
                GenesisConfigService::get_u64(SIGNUP_REWARD_AMOUNT_PHASE1_KEY.into())
                    .await?
                    .unwrap(),
            )
        }*/
    }

    /// Get current referral; reward amount based on consensus rules, genesis config and blockchain data
    pub async fn get_referral_reward_amount(&self) -> Result<u64> {
        Ok(
            GenesisConfigService::get_u64(REFERRAL_REWARD_AMOUNT_PHASE1_KEY.into())
                .await?
                .unwrap(),
        )
        /*
        let rewards_alloc_phase1 =
            GenesisConfigService::get_u64(REFERRAL_REWARD_ALLOCATION_PHASE1_KEY.into())
                .await?
                .unwrap();

        let rewards_alloc_phase2 =
            GenesisConfigService::get_u64(REFERRAL_REWARD_ALLOCATION_PHASE2_KEY.into())
                .await?
                .unwrap();

        if self.stats.referral_rewards_amount > rewards_alloc_phase2 + rewards_alloc_phase1 {
            Ok(0)
        } else if self.stats.referral_rewards_amount > rewards_alloc_phase1 {
            Ok(
                GenesisConfigService::get_u64(REFERRAL_REWARD_AMOUNT_PHASE2_KEY.into())
                    .await?
                    .unwrap(),
            )
        } else {
            Ok(
                GenesisConfigService::get_u64(REFERRAL_REWARD_AMOUNT_PHASE1_KEY.into())
                    .await?
                    .unwrap(),
            )
        }*/
    }

    /// Return true iff transaction should be subsidised by the protocol
    pub async fn should_subsidise_transaction_fee(
        &self,
        user_nonce: u64,
        fee_amount: u64,
        tx_type: TransactionType,
    ) -> Result<bool> {
        if fee_amount
            > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_AMOUNT_KEY.into())
                .await?
                .unwrap()
        {
            // tx fee too high for subsidy
            info!("tx fee too high for subsidy");
            return Ok(false);
        }

        if user_nonce
            > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY.into())
                .await?
                .unwrap()
        {
            return Ok(false);
        }

        if self.stats.fee_subs_amount
            <= GenesisConfigService::get_u64(TX_FEE_SUBSIDY_ALLOCATION_PHASE1_KEY.into())
                .await?
                .unwrap()
        {
            // we are in phase 1 subsidies, validate fee is below the max phase1 subsidy amount
            if fee_amount
                > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_AMOUNT_PHASE1_KEY.into())
                    .await?
                    .unwrap()
            {
                return Ok(false);
            }

            return Ok(true);
        }

        // we are beyond phase 1 subsidies, only signup txs up to max fee are subsided
        if tx_type != TransactionType::NewUserV1 {
            return Ok(false);
        }

        // Validate signup tx fee is below max subsidy amount
        if fee_amount
            > GenesisConfigService::get_u64(TX_FEE_SUBSIDY_MAX_AMOUNT_KEY.into())
                .await?
                .unwrap()
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// Gets the current block reward for block producer based on block height in KCents
    pub async fn get_block_reward_amount(&self, block_height: u64) -> Result<u64> {
        if block_height
            > GenesisConfigService::get_u64(BLOCK_REWARDS_LAST_BLOCK.into())
                .await?
                .unwrap()
        {
            // no more block rewards
            return Ok(0);
        }

        Ok(GenesisConfigService::get_u64(BLOCK_REWARDS_AMOUNT.into())
            .await?
            .unwrap())
    }
}
