// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::BLOCKS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::*;
use db::db_service::{DatabaseService, ReadItem};
use db::types::IntDbKey;
use prost::Message;
use xactor::*;

#[message(result = "Result<Vec<Block>>")]
pub(crate) struct GetBlocks {
    pub(crate) from_height: u64,
    pub(crate) to_height: u64,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetBlocks> for BlockChainService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetBlocks) -> Result<Vec<Block>> {
        let mut res = vec![];
        for h in msg.from_height..=msg.to_height {
            if let Some(event) = self.get_block_by_height(h).await? {
                res.push(event);
            }
        }
        Ok(res)
    }
}

impl BlockChainService {
    async fn get_block_by_height(&self, height: u64) -> Result<Option<Block>> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: IntDbKey::from(height).0,
            cf: BLOCKS_COL_FAMILY,
        })
        .await?
        {
            Ok(Some(Block::decode(data.0)?))
        } else {
            Ok(None)
        }
    }
}
