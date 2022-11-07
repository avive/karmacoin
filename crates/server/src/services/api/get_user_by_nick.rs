// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::db_config_service::{NICKS_COL_FAMILY, USERS_COL_FAMILY};
use base::karma_coin::karma_coin_api::{GetUserInfoByNickRequest, GetUserInfoByNickResponse};
use base::karma_coin::karma_coin_core_types::User;
use crate::services::api::api_service::ApiService;
use prost::Message;

#[message(result = "Result<GetUserInfoByNickResponse>")]
pub(crate) struct GetUserInfoByNick(pub (crate) GetUserInfoByNickRequest);

#[async_trait::async_trait]
impl Handler<GetUserInfoByNick> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByNick,
    ) -> Result<GetUserInfoByNickResponse> {

        // lookup accountId by nickname
        match DatabaseService::read(ReadItem {
            key: Bytes::from(msg.0.nickname.as_bytes().to_vec()),
            cf: NICKS_COL_FAMILY,
        }).await? {
            Some(data) => {
                // lookup user from db by accountId
                match DatabaseService::read(ReadItem {
                    key: data.0,
                    cf: USERS_COL_FAMILY
                }).await? {
                    // fetch user by account id from db
                    Some(user_data) => {
                        Ok(GetUserInfoByNickResponse {
                            user: Some(User::decode(user_data.0.as_ref())?),
                        })
                    },
                    None => Ok(GetUserInfoByNickResponse { user: None })
                }
            },
            None => Ok(GetUserInfoByNickResponse { user: None })
        }
    }
}