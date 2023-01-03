// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::api::api_service::ApiService;
use anyhow::Result;
use base::genesis_config_service::{GenesisConfigService, CHAR_TRAITS_KEY};
use base::karma_coin::karma_coin_api::{GetCharTraitsRequest, GetCharTraitsResponse};
use base::karma_coin::karma_coin_core_types::CharTrait;
use xactor::*;

#[message(result = "Result<GetCharTraitsResponse>")]
pub(crate) struct GetCharTraits(pub(crate) GetCharTraitsRequest);

#[async_trait::async_trait]
impl Handler<GetCharTraits> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCharTraits,
    ) -> Result<GetCharTraitsResponse> {
        if let Some(traits) = self.char_traits.as_ref() {
            return Ok(GetCharTraitsResponse {
                char_traits: traits.clone(),
            });
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

        // cache results
        self.char_traits = Some(traits.clone());

        Ok(GetCharTraitsResponse {
            char_traits: traits,
        })
    }
}
