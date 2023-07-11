// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::stats::{get_stats, GetStats, WriteStats};
use crate::services::db_config_service::{LEADER_BOARD_COL_FAMILY, USERS_COL_FAMILY};
use crate::Tokenomics;
use anyhow::Result;
use base::genesis_config_service::{
    GenesisConfigService, KARMA_REWARD_MAX_USERS_KEY, KARMA_REWARD_PERIOD_MINUTES,
    KARMA_REWARD_TRAIT_ID,
};
use base::karma_coin::karma_coin_api::GetBlockchainDataRequest;
use base::karma_coin::karma_coin_core_types::{LeaderboardEntry, User};
use bytes::Bytes;
use db::db_service::{
    DataItem, DatabaseService, DeleteAllItems, ReadAllItems, ReadItem, WriteItem,
};
use prost::Message;
use rand::prelude::*;
use rand_core::OsRng;
use tokio::spawn;
use tokio_schedule::{every, Job};
use xactor::*;

/// A simple transactions pool service
/// This service is used to store transactions that are not yet included in a block
#[derive(Debug, Clone, Default)]
pub(crate) struct KarmaRewardsService {}

impl Service for KarmaRewardsService {}

#[async_trait::async_trait]
impl Actor for KarmaRewardsService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("started. Registering karma rewards periodic task...");

        let task_period_minutes = GenesisConfigService::get_u64(KARMA_REWARD_PERIOD_MINUTES.into())
            .await?
            .unwrap() as u32;

        let task = every(task_period_minutes).minutes().perform(|| async {
            let service = KarmaRewardsService::from_registry().await;
            if service.is_err() {
                error!("VerifierService not available");
                return;
            }
            info!("Starting periodic karma rewards processing task...");
            match service.unwrap().call(ProcessKarmaRewards).await {
                Ok(res) => match res {
                    Ok(_) => info!("Karma Rewards processing task completed"),
                    Err(e) => error!("Karma Rewards processing task error: {}", e),
                },
                Err(e) => error!("Error running invites task: {}", e),
            }
        });
        spawn(task);

        Ok(())
    }
}

#[message(result = "Result<()>")]
pub(crate) struct ProcessKarmaRewards;

/// Assign Karma Rewards based on current leader board
#[async_trait::async_trait]
impl Handler<ProcessKarmaRewards> for KarmaRewardsService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: ProcessKarmaRewards) -> Result<()> {
        info!("processing karma rewards task...");
        let stats = get_stats().await?;
        let tokenomics = Tokenomics::new(stats.clone());
        let reward_amount = tokenomics.get_karma_coin_reward_amount().await?;
        if reward_amount == 0 {
            info!("karma rewards depleted");
            return DatabaseService::delete_all(DeleteAllItems {
                cf: LEADER_BOARD_COL_FAMILY,
            })
            .await;
        }
        let max_winners = usize::try_from(
            GenesisConfigService::get_u64(KARMA_REWARD_MAX_USERS_KEY.into())
                .await?
                .ok_or_else(|| anyhow::anyhow!("Karma Reward Max Users not set"))?,
        )?;

        let data = DatabaseService::read_all_items(ReadAllItems {
            from_key: None,
            max_results: 0,
            cf: LEADER_BOARD_COL_FAMILY,
        })
        .await?;

        let mut rng = OsRng;
        let winners_data: Vec<_> = data.items.choose_multiple(&mut rng, max_winners).collect();
        info!(
            "Selected {} winners for karma rewards from {} total in leaderboard",
            winners_data.len(),
            data.items.len()
        );

        if !winners_data.is_empty() {
            let mut total_reward_amount = 0;
            for item in winners_data.iter() {
                let entry = LeaderboardEntry::decode(item.1.value.as_ref())?;

                match self.process_winner(&entry, reward_amount).await {
                    Ok(amount) => total_reward_amount += amount,
                    Err(e) => error!("Error processing karma reward winner: {}", e),
                }
            }

            let service = BlockChainService::from_registry().await?;

            let mut resp = service
                .call(GetStats(GetBlockchainDataRequest {}))
                .await??;

            let mut stats = resp.stats.as_mut().unwrap();
            stats.karma_rewards_amount += total_reward_amount;

            service.call(WriteStats(stats.clone())).await??;

            info!("deleting leaderboard");
            DatabaseService::delete_all(DeleteAllItems {
                cf: LEADER_BOARD_COL_FAMILY,
            })
            .await
        } else {
            info!("no winners to process");
            Ok(())
        }
    }
}

impl KarmaRewardsService {
    /// Process an award winner - helper method
    async fn process_winner(&self, entry: &LeaderboardEntry, reward_amount: u64) -> Result<u64> {
        // load user
        let account_id = entry.account_id.as_ref().unwrap().data.clone();
        let user = DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;
        let mut user = User::decode(user.0.as_ref())?;

        // make sure user has not already been rewarded
        if user.get_trait_score(KARMA_REWARD_TRAIT_ID, 0) > 0 {
            // user already got a karma reward
            return Ok(0);
        }

        // make sure this is not a migrated account
        if user.user_name.ends_with("[old account]") {
            // migrated account
            return Ok(0);
        }

        // assign karma reward
        user.inc_trait_score(KARMA_REWARD_TRAIT_ID, 0);

        // give reward
        user.balance += reward_amount;

        info!(
            "rewarding {} with {} karma coins and adding reward trait",
            user.user_name, reward_amount
        );

        // todo: emit event here...

        // persist user
        let mut buf = Vec::with_capacity(user.encoded_len());
        user.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account_id.clone()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        Ok(reward_amount)
    }
}
