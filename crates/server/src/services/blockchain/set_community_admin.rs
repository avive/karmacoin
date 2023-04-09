// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_api::{
    SetCommunityAdminData, SetCommunityAdminRequest, SetCommunityAdminResponse,
};
use base::karma_coin::karma_coin_core_types::{CommunityMembership, User};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::Verifier;
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
            .ok_or_else(|| anyhow::anyhow!("missing from account id"))?;

        let account_id_short = short_hex_string(account_id.data.as_ref());

        let pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_ref())?;
        pub_key.verify(
            req.data.as_ref(),
            &Signature::from_bytes(req.signature.as_ref())?,
        )?;

        let data = SetCommunityAdminData::decode(req.data.as_ref())?;

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
            .get_community_membership(data.community_id)
            .ok_or_else(|| {
                anyhow!(
                    "user {} is not a member of community {}",
                    account_id_short,
                    data.community_id
                )
            })?;

        if !membership.is_admin {
            return Err(anyhow!(
                "user {} is not an admin of community {}",
                account_id_short,
                data.community_id
            ));
        }

        // user signed the message and is account admin in group

        let invited_account_id = data
            .target_account_id
            .ok_or_else(|| anyhow::anyhow!("missing from account id"))?;

        let invited_account_id_short = short_hex_string(invited_account_id.data.as_ref());

        let mut invited = match DatabaseService::read(ReadItem {
            key: Bytes::from(invited_account_id.data.clone()),
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

        match invited.get_community_membership(data.community_id) {
            Some(membership) => membership.is_admin = true,
            None => invited.community_memberships.push(CommunityMembership {
                community_id: data.community_id,
                karma_score: 1,
                is_admin: true,
            }),
        }

        // Update invited onchain data
        let mut buf = Vec::with_capacity(invited.encoded_len());
        invited.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(invited.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        info!(
            "{} set as admin for {}",
            invited_account_id_short, data.community_id
        );

        Ok(SetCommunityAdminResponse {})
    }
}
