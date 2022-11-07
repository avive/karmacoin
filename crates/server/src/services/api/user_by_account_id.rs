// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::db_config_service::{NICKS_COL_FAMILY, USERS_COL_FAMILY};
use base::karma_coin::karma_coin_api::{GetUserInfoByAccountRequest, GetUserInfoByAccountResponse, GetUserInfoByNickRequest, GetUserInfoByNickResponse};
use base::karma_coin::karma_coin_core_types::User;
use crate::services::api::api_service::ApiService;
use prost::Message;

#[message(result = "Result<GetUserInfoByAccountResponse>")]
pub(crate) struct GetUserInfoByAccountId(pub (crate) GetUserInfoByAccountRequest);

#[async_trait::async_trait]
impl Handler<GetUserInfoByAccountId> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByAccountId,
    ) -> Result<GetUserInfoByAccountResponse> {

        if msg.0.account_id.is_none() {
            return Err(anyhow!("missing phone number from request"));
        };

       match DatabaseService::read(ReadItem {
                    key: Bytes::from(msg.0.account_id.unwrap().data),
                    cf: USERS_COL_FAMILY
                }).await? {
           // fetch user by account id from db
           Some(user_data) => {
               Ok(GetUserInfoByAccountResponse {
                   user: Some(User::decode(user_data.0.as_ref())?),
               })
           },
           None => Ok(GetUserInfoByAccountResponse { user: None })
       }
    }
}