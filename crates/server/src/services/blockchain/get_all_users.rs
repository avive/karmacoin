// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_api::{GetAllUsersRequest, GetAllUsersResponse};
use base::karma_coin::karma_coin_core_types::User;
use db::db_service::{DatabaseService, ReadAllItems};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetAllUsersResponse>")]
pub(crate) struct GetAllUsers(pub(crate) GetAllUsersRequest);

#[async_trait::async_trait]
impl Handler<GetAllUsers> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetAllUsers,
    ) -> Result<GetAllUsersResponse> {
        let mut users = vec![];

        let data = DatabaseService::read_all_items(ReadAllItems {
            from_key: None,
            max_results: 0,
            cf: USERS_COL_FAMILY,
        })
        .await?;

        info!(
            "We got {} users (pre community filtering)",
            data.items.len()
        );
        let community_id = msg.0.community_id;

        for item in data.items.iter() {
            match User::decode(item.1.value.as_ref()) {
                Ok(user) => {
                    if community_id != 0 && !user.is_community_member(community_id) {
                        continue;
                    }
                    info!("User: {}", user);
                    users.push(user);
                }
                Err(e) => {
                    error!("Error decoding user: {}", e);
                }
            }
        }

        info!("returning {} users", users.len());

        Ok(GetAllUsersResponse { users })
    }
}
