// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::base::hex_utils::short_hex_string;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::LEADER_BOARD_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::{AccountId, LeaderboardEntry, User};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;

impl BlockChainService {
    /// Update leaderboard with user - insert or update entry
    pub(crate) async fn leader_board_upsert(
        &mut self,
        user: &User,
        chart_trait_id: u32,
    ) -> Result<()> {
        let account_id = user.account_id.as_ref().unwrap().data.clone();

        let mut entry = match DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.clone()),
            cf: LEADER_BOARD_COL_FAMILY,
        })
        .await?
        {
            // fetch user by account id from db
            Some(entry) => {
                info!(
                    "found leader board entry on-chain by account id: {}",
                    short_hex_string(account_id.as_ref())
                );
                LeaderboardEntry::decode(entry.0.as_ref())?
            }
            None => {
                info!(
                    "no leaderboard on-chain found for account id: {}",
                    short_hex_string(account_id.as_ref())
                );
                LeaderboardEntry {
                    user_name: user.user_name.clone(),
                    account_id: Some(AccountId {
                        data: account_id.clone(),
                    }),
                    score: 0,
                    char_traits_ids: vec![],
                }
            }
        };

        // update the entry
        entry.score += 1;
        if !chart_trait_id != 0 {
            entry.char_traits_ids.push(chart_trait_id);
        }

        // write updated entry to db
        let mut buf = Vec::with_capacity(entry.encoded_len());
        entry.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account_id),
                value: Bytes::from(buf),
            },
            cf: LEADER_BOARD_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        Ok(())
    }
}
