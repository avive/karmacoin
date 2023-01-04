// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use anyhow::Result;
use base::genesis_config_service::{GenesisConfigService, CHAR_TRAITS_KEY};
use base::karma_coin::karma_coin_api::{GetGenesisDataRequest, GetGenesisDataResponse};
use base::karma_coin::karma_coin_core_types::CharTrait;
use xactor::*;

#[message(result = "Result<GetGenesisDataResponse>")]
pub(crate) struct GetGenesisData {
    pub(crate) request: GetGenesisDataRequest,
}

#[async_trait::async_trait]
impl Handler<GetGenesisData> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetGenesisData,
    ) -> Result<GetGenesisDataResponse> {
        let _req = msg.request;
        let char_traits = self.get_char_traits().await?;

        Ok(GetGenesisDataResponse {
            net_id: 0,
            net_name: "".to_string(),
            genesis_time: 0,
            signup_reward_phase1_alloc: 0,
            signup_reward_phase2_alloc: 0,
            signup_reward_phase1_amount: 0,
            signup_reward_phase2_amount: 0,
            signup_reward_phase3_start: 0,
            referral_reward_phase1_alloc: 0,
            referral_reward_phase2_alloc: 0,
            referral_reward_phase1_amount: 0,
            referral_reward_phase2_amount: 0,
            tx_fee_subsidy_max_per_user: 0,
            tx_fee_subsidies_alloc: 0,
            tx_fee_subsidy_max_amount: 0,
            block_reward_amount: 0,
            block_reward_last_block: 0,
            karma_reward_amount: 0,
            karma_reward_alloc: 0,
            treasury_premint_amount: 0,
            char_traits,
            verifiers: vec![],
        })
    }
}

impl BlockChainService {
    /// Returns all supported char traits from genesis data
    async fn get_char_traits(&mut self) -> Result<Vec<CharTrait>> {
        if let Some(traits) = self.char_traits.as_ref() {
            return Ok(traits.clone());
        }

        let mut traits = vec![];
        for (id, name) in GenesisConfigService::get_map(CHAR_TRAITS_KEY.into())
            .await?
            .unwrap()
        {
            traits.push(CharTrait::new(
                id.parse().unwrap(),
                name.into_string().unwrap().as_str(),
            ));
        }

        self.char_traits = Some(traits.clone());

        Ok(traits)
    }
}
