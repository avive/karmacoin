// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use base::karma_coin::karma_coin_blockchain::{GetHeadHeightRequest, GetHeadHeightResponse};
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{BLOCK_TIP_KEY, NET_SETTINGS_COL_FAMILY};

#[message(result = "Result<GetHeadHeightResponse>")]
pub(crate) struct GetHeadHeight(pub GetHeadHeightRequest);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetHeadHeight> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetHeadHeight,
    ) -> Result<GetHeadHeightResponse> {
        get_tip().await
    }
}

/// Helper function to get the tip of the blockchain
pub(crate) async fn get_tip() -> Result<GetHeadHeightResponse> {
    if let Some(data) = DatabaseService::read(ReadItem {
        key: Bytes::from(BLOCK_TIP_KEY.as_bytes()),
        cf: NET_SETTINGS_COL_FAMILY
    }).await? {
        let height = BigEndian::read_u64(data.0.as_ref());
        Ok(GetHeadHeightResponse {
            height
        })
    } else {
        Ok(GetHeadHeightResponse {
            height: 0
        })
    }
}


