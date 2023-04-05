// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::LEADER_BOARD_COL_FAMILY;
use anyhow::Result;
use base::genesis_config_service::{GenesisConfigService, KARMA_REWARDS_ELIGIBILITY};
use base::karma_coin::karma_coin_api::{GetLeaderBoardRequest, GetLeaderBoardResponse};
use base::karma_coin::karma_coin_core_types::LeaderboardEntry;
use db::db_service::{DatabaseService, ReadAllItems};
use prost::Message;
use xactor::*;

#[message(result = "Result<GetLeaderBoardResponse>")]
pub(crate) struct GetLeaderBoard(pub(crate) GetLeaderBoardRequest);

#[async_trait::async_trait]
impl Handler<GetLeaderBoard> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetLeaderBoard,
    ) -> Result<GetLeaderBoardResponse> {
        let min_appreciations = usize::try_from(
            GenesisConfigService::get_u64(KARMA_REWARDS_ELIGIBILITY.into())
                .await?
                .ok_or_else(|| anyhow::anyhow!("Karma Reward Max Users not set"))?,
        )?;

        let mut leaderboard_entries = vec![];

        let data = DatabaseService::read_all_items(ReadAllItems {
            from_key: None,
            max_results: 0,
            cf: LEADER_BOARD_COL_FAMILY,
        })
        .await?;

        for item in data.items.iter() {
            let entry = LeaderboardEntry::decode(item.1.value.as_ref())?;
            if entry.score >= min_appreciations as u32 {
                leaderboard_entries.push(entry);
            }
        }

        Ok(GetLeaderBoardResponse {
            leaderboard_entries,
        })
    }
}
