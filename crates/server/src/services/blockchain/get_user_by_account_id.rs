// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_api::{GetUserInfoByAccountRequest, GetUserInfoByAccountResponse};
use base::karma_coin::karma_coin_core_types::User;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetUserInfoByAccountResponse>")]
pub(crate) struct GetUserInfoByAccountId(pub(crate) GetUserInfoByAccountRequest);

#[async_trait::async_trait]
impl Handler<GetUserInfoByAccountId> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByAccountId,
    ) -> Result<GetUserInfoByAccountResponse> {
        if msg.0.account_id.is_none() {
            return Err(anyhow!("missing account id from request"));
        };

        match DatabaseService::read(ReadItem {
            key: Bytes::from(msg.0.account_id.as_ref().unwrap().data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            // fetch user by account id from db
            Some(user_data) => {
                info!(
                    "found user on-chain by account id: {}",
                    short_hex_string(user_data.0.as_ref())
                );
                Ok(GetUserInfoByAccountResponse {
                    user: Some(User::decode(user_data.0.as_ref())?),
                })
            }
            None => {
                info!(
                    "no user on-chain found account id: {}",
                    short_hex_string(msg.0.account_id.unwrap().data.as_ref())
                );
                Ok(GetUserInfoByAccountResponse { user: None })
            }
        }
    }
}
