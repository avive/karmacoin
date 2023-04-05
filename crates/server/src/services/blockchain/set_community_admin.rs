// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_api::{SetCommunityAdminRequest, SetCommunityAdminResponse};
use base::karma_coin::karma_coin_core_types::User;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<SetCommunityAdminResponse>")]
pub(crate) struct SetCommunityAdmin(pub(crate) SetCommunityAdminRequest);

#[async_trait::async_trait]
impl Handler<SetCommunityAdmin> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SetCommunityAdmin,
    ) -> Result<SetCommunityAdminResponse> {
        let req = msg.0;

        let account_id = req
            .from_account_id
            .ok_or_else(|| anyhow!("missing account id"))?;

        // check sender signature

        let account_id_short = short_hex_string(account_id.data.as_ref());

        // lookup accountId by user name
        let mut user = match DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            Some(data) => User::decode(data.0.as_ref())?,
            None => {
                info!("no user for {}", account_id_short);
                return Err(anyhow!("no user for {}", account_id_short));
            }
        };

        let membership = user
            .get_community_membership(req.community_id)
            .ok_or_else(|| {
                anyhow!(
                    "user {} is not a member of community {}",
                    account_id_short,
                    req.community_id
                )
            })?;

        if !membership.is_admin {
            return Err(anyhow!(
                "user {} is not an admin of community {}",
                account_id_short,
                req.community_id
            ));
        }

        unimplemented!()
    }
}
