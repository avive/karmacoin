// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use anyhow::Result;
use base::karma_coin::karma_coin_api::{
    GetUserInfoByUserNameRequest, GetUserInfoByUserNameResponse,
};
use base::karma_coin::karma_coin_core_types::User;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetUserInfoByUserNameResponse>")]
pub(crate) struct GetUserInfoByUserName(pub(crate) GetUserInfoByUserNameRequest);

#[async_trait::async_trait]
impl Handler<GetUserInfoByUserName> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByUserName,
    ) -> Result<GetUserInfoByUserNameResponse> {
        // lookup accountId by nickname
        match DatabaseService::read(ReadItem {
            key: Bytes::from(msg.0.user_name.as_bytes().to_vec()),
            cf: USERS_NAMES_COL_FAMILY,
        })
        .await?
        {
            Some(data) => {
                // lookup user from db by accountId
                match DatabaseService::read(ReadItem {
                    key: data.0,
                    cf: USERS_COL_FAMILY,
                })
                .await?
                {
                    // fetch user by account id from db
                    Some(user_data) => Ok(GetUserInfoByUserNameResponse {
                        user: Some(User::decode(user_data.0.as_ref())?),
                    }),
                    None => Ok(GetUserInfoByUserNameResponse { user: None }),
                }
            }
            None => Ok(GetUserInfoByUserNameResponse { user: None }),
        }
    }
}
