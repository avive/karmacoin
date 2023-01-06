// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::BLOCK_EVENTS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use db::types::IntDbKey;
use prost::Message;
use xactor::*;

// add handler to get block event for a block number

#[message(result = "Result<Vec<BlockEvent>>")]
pub(crate) struct GetBlocksEvents {
    pub(crate) from_height: u64,
    pub(crate) to_height: u64,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetBlocksEvents> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetBlocksEvents,
    ) -> Result<Vec<BlockEvent>> {
        let mut res = vec![];
        for h in msg.from_height..=msg.to_height {
            if let Some(event) = self.get_block_event_by_height(h).await? {
                res.push(event);
            }
        }
        Ok(res)
    }
}

// todo: add handler to get block events from a block number to another block number and use rocks
// db range to get all events in one operation

impl BlockChainService {
    /// Get block event for a block at provided height
    pub(crate) async fn get_block_event_by_height(
        &self,
        height: u64,
    ) -> Result<Option<BlockEvent>> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: IntDbKey::from(height).0,
            cf: BLOCK_EVENTS_COL_FAMILY,
        })
        .await?
        {
            let block_event = BlockEvent::decode(data.0.as_ref())?;
            Ok(Some(block_event))
        } else {
            Ok(None)
        }
    }

    /// emit a new block event
    pub(crate) async fn emit_block_event(&self, event: &BlockEvent) -> Result<()> {
        let mut buf = Vec::with_capacity(event.encoded_len());
        event.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: IntDbKey::from(event.height).0,
                value: Bytes::from(buf),
            },
            cf: BLOCK_EVENTS_COL_FAMILY,
            ttl: 0, // todo: set ttl based on this node being archive or full
        })
        .await?;

        info!("Block event emitted: {}", event);

        Ok(())
    }
}
