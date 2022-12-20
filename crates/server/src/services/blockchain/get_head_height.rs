// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{BLOCK_TIP_KEY, NET_SETTINGS_COL_FAMILY};

#[message(result = "Result<u64>")]
pub(crate) struct GetHeadHeight;

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetHeadHeight> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetHeadHeight,
    ) -> Result<u64> {
        get_tip().await
    }
}

/// Helper function to get the tip of the blockchain
pub(crate) async fn get_tip() -> Result<u64> {
    if let Some(data) = DatabaseService::read(ReadItem {
        key: Bytes::from(BLOCK_TIP_KEY.as_bytes()),
        cf: NET_SETTINGS_COL_FAMILY
    }).await? {
        Ok(BigEndian::read_u64(data.0.as_ref()))
    } else {
        Ok(0)
    }
}


