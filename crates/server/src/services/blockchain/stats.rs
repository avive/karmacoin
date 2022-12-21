// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_api::{GetBlockchainDataRequest, GetBlockchainDataResponse};
use base::karma_coin::karma_coin_core_types::BlockchainStats;
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{BLOCKCHAIN_DATA_COL_FAMILY, CHAIN_STATS_KEY};

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
        Ok(GetBlockchainDataResponse {
            stats: Some(stats),
        })
    }
}

/// Helper function to get the current blockchain stats
pub(crate) async fn get_stats() -> Result<BlockchainStats> {
    if let Some(data) = DatabaseService::read(ReadItem {
        key: Bytes::from(CHAIN_STATS_KEY.as_bytes()),
        cf: BLOCKCHAIN_DATA_COL_FAMILY
    }).await? {
        let stats = BlockchainStats::decode(data.0.as_ref())?;
        Ok(stats)
    } else {
        Ok(BlockchainStats::new())
    }
}

/// Helper function to write stats to the db
pub(crate) async fn write_stats(stats:BlockchainStats) -> Result<()> {
    let mut buf = Vec::with_capacity(stats.encoded_len());
    stats.encode(&mut buf)?;
    DatabaseService::write(
        WriteItem {
            data: DataItem {
                key: Bytes::from(CHAIN_STATS_KEY.as_bytes()),
                value: Bytes::from(buf)
            },
            cf: BLOCKCHAIN_DATA_COL_FAMILY,
            ttl: 0,
        }).await
}

