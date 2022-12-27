// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::api::api_service::ApiService;
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, USERS_COL_FAMILY};
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_api::{GetUserInfoByNumberRequest, GetUserInfoByNumberResponse};
use base::karma_coin::karma_coin_core_types::User;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetUserInfoByNumberResponse>")]
pub(crate) struct GetUserInfoByNumber(pub(crate) GetUserInfoByNumberRequest);

#[async_trait::async_trait]
impl Handler<GetUserInfoByNumber> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByNumber,
    ) -> Result<GetUserInfoByNumberResponse> {
        if msg.0.mobile_number.is_none() {
            return Err(anyhow!("missing phone number from request"));
        };

        // lookup accountId by nickname
        match DatabaseService::read(ReadItem {
            key: Bytes::from(msg.0.mobile_number.unwrap().number.as_bytes().to_vec()),
            cf: MOBILE_NUMBERS_COL_FAMILY,
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
                    Some(user_data) => Ok(GetUserInfoByNumberResponse {
                        user: Some(User::decode(user_data.0.as_ref())?),
                    }),
                    None => Ok(GetUserInfoByNumberResponse { user: None }),
                }
            }
            None => Ok(GetUserInfoByNumberResponse { user: None }),
        }
    }
}
