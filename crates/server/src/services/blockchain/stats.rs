// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{BLOCKCHAIN_DATA_COL_FAMILY, CHAIN_AGG_DATA_KEY};
use anyhow::Result;
use base::karma_coin::karma_coin_api::{GetBlockchainDataRequest, GetBlockchainDataResponse};
use base::karma_coin::karma_coin_core_types::{Block, BlockEvent, BlockchainStats, FeeType};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use xactor::*;

impl BlockChainService {
    /// Update blockchain stats with new block data and store in db
    pub(crate) async fn update_blockchain_stats(
        mut stats: BlockchainStats,
        block_event: &BlockEvent,
        block: &Block,
    ) -> Result<()> {
        stats.last_block_time = block.time;
        stats.tip_height += 1;
        stats.transactions_count += block.transactions_hashes.len() as u64;
        stats.users_count += block_event.signups_count;

        stats.payments_transactions_count += block_event.payments_count;
        stats.signup_rewards_amount += block_event.signup_rewards_amount;
        stats.signup_rewards_count += block_event.signups_count;
        stats.referral_rewards_amount += block_event.referral_rewards_amount;
        stats.referral_rewards_count += block_event.referral_rewards_count;

        stats.fees_amount += block_event.fees_amount;

        stats.minted_amount += block_event.reward
            + block_event.referral_rewards_amount
            + block_event.signup_rewards_amount;

        for tx_event in block_event.transactions_events.iter() {
            if tx_event.fee_type == FeeType::Mint as i32 {
                stats.fee_subs_count += 1;
                stats.fee_subs_amount += tx_event.fee;
                stats.minted_amount += tx_event.fee;
            }
        }

        write_stats(stats).await
    }
}

#[message(result = "Result<GetBlockchainDataResponse>")]
pub(crate) struct GetStats(pub(crate) GetBlockchainDataRequest);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetStats> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetStats,
    ) -> Result<GetBlockchainDataResponse> {
        let stats = get_stats().await?;
        Ok(GetBlockchainDataResponse { stats: Some(stats) })
    }
}

/// Helper function to get the current blockchain stats
pub(crate) async fn get_stats() -> Result<BlockchainStats> {
    if let Some(data) = DatabaseService::read(ReadItem {
        key: Bytes::from(CHAIN_AGG_DATA_KEY.as_bytes()),
        cf: BLOCKCHAIN_DATA_COL_FAMILY,
    })
    .await?
    {
        let stats = BlockchainStats::decode(data.0.as_ref())?;
        Ok(stats)
    } else {
        Ok(BlockchainStats::new())
    }
}

/// Helper function to write stats to the db
pub(crate) async fn write_stats(stats: BlockchainStats) -> Result<()> {
    let mut buf = Vec::with_capacity(stats.encoded_len());
    stats.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(CHAIN_AGG_DATA_KEY.as_bytes()),
            value: Bytes::from(buf),
        },
        cf: BLOCKCHAIN_DATA_COL_FAMILY,
        ttl: 0,
    })
    .await
}
