// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
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
impl Handler<GetUserInfoByNumber> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetUserInfoByNumber,
    ) -> Result<GetUserInfoByNumberResponse> {
        if msg.0.mobile_number.is_none() {
            return Err(anyhow!("missing phone number from request"));
        };

        // lookup accountId by phone number
        match DatabaseService::read(ReadItem {
            key: Bytes::from(
                msg.0
                    .mobile_number
                    .as_ref()
                    .unwrap()
                    .number
                    .as_bytes()
                    .to_vec(),
            ),
            cf: MOBILE_NUMBERS_COL_FAMILY,
        })
        .await?
        {
            Some(data) => {
                info!(
                    "found account id for phone number {} in phone numbers index",
                    msg.0.mobile_number.as_ref().unwrap().number
                );

                match DatabaseService::read(ReadItem {
                    key: data.0,
                    cf: USERS_COL_FAMILY,
                })
                .await?
                {
                    // fetch user by account id from db
                    Some(user_data) => {
                        info!("Found user for this number in db");
                        Ok(GetUserInfoByNumberResponse {
                            user: Some(User::decode(user_data.0.as_ref())?),
                        })
                    }
                    None => {
                        info!("No user found for this number in db");
                        Ok(GetUserInfoByNumberResponse { user: None })
                    }
                }
            }

            None => {
                info!("No account id found for this number in phone numbers index");
                Ok(GetUserInfoByNumberResponse { user: None })
            }
        }
    }
}
