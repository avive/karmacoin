// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::BLOCK_EVENTS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, WriteItem};
use db::types::IntDbKey;
use prost::Message;

impl BlockChainService {
    /// emit a transaction processing event
    pub(crate) async fn emit_block_event(event: &BlockEvent) -> Result<()> {
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

        info!("Block event emitted: {:?}", event);

        Ok(())
    }
}
