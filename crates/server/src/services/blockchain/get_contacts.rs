// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use anyhow::Result;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_api::{GetContactsRequest, GetContactsResponse};
use base::karma_coin::karma_coin_core_types::{Contact, User};
use db::db_service::{DatabaseService, ReadAllItems, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetContactsResponse>")]
pub(crate) struct GetContacts(pub(crate) GetContactsRequest);

#[async_trait::async_trait]
impl Handler<GetContacts> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetContacts,
    ) -> Result<GetContactsResponse> {
        // todo: use cache on hourly basis per prefix to avoid load on db and too many reads
        // this is an expensive operation

        let community_id = msg.0.community_id;

        // compute prefix
        let from = if msg.0.prefix.is_empty() {
            None
        } else {
            Some(msg.0.prefix.clone())
        };

        let data = DatabaseService::read_all_items(ReadAllItems {
            from_key: from,
            max_results: 0,
            cf: USERS_NAMES_COL_FAMILY,
        })
        .await?;

        let mut contacts = vec![];

        info!("got {} items from db", data.items.len());

        for item in data.items.iter() {
            info!("account key: {}", short_hex_string(item.1.value.as_ref()));
            let user_data = match DatabaseService::read(ReadItem {
                key: item.1.value.clone(),
                cf: USERS_COL_FAMILY,
            })
            .await?
            {
                // fetch user by account id from db
                Some(user_data) => Some(User::decode(user_data.0.as_ref())?),
                None => {
                    error!(
                        "User not found in db from user names index for {}",
                        short_hex_string(item.0.as_ref())
                    );
                    None
                }
            };

            if let Some(mut user) = user_data {
                if user.mobile_number.is_none() {
                    continue;
                }

                // if caller asked for only members of a community and user is not a member of it
                // then skip this user
                if community_id != 0 && user.get_community_membership(community_id).is_none() {
                    continue;
                }

                // create contact for user
                contacts.push(Contact {
                    user_name: user.user_name.to_string(),
                    account_id: Some(user.account_id.unwrap()),
                    mobile_number: Some(user.mobile_number.unwrap()),
                    community_memberships: user.community_memberships,
                })
            }
        }

        Ok(GetContactsResponse { contacts })
    }
}
