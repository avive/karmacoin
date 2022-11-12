// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::db_config_service::{DB_SUPPORTED_TRAITS_KEY, NET_SETTINGS_COL_FAMILY};
use base::karma_coin::karma_coin_api::{GetCharTraitsRequest, GetCharTraitsResponse};
use base::karma_coin::karma_coin_core_types::{Traits};
use crate::services::api::api_service::ApiService;
use prost::Message;

#[message(result = "Result<GetCharTraitsResponse>")]
pub(crate) struct GetCharTraits(pub (crate) GetCharTraitsRequest);

#[async_trait::async_trait]
impl Handler<GetCharTraits> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetCharTraits,
    ) -> Result<GetCharTraitsResponse> {

        // lookup accountId by nickname
        match DatabaseService::read(ReadItem {
            key: Bytes::from(DB_SUPPORTED_TRAITS_KEY.as_bytes()),
            cf: NET_SETTINGS_COL_FAMILY,
        }).await? {
            Some(data) => {

                let traits = Traits::decode(data.0.as_ref())?;
                Ok(GetCharTraitsResponse {
                    trait_names: traits.named_traits
                })
            },
            None => Ok(GetCharTraitsResponse {
                trait_names: vec![]
                 })
        }
    }
}