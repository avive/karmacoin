// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, WriteItem};
// use config::Config;

use crate::services::db_config_service::{
    DbConfigService, BLOCKCHAIN_DATA_COL_FAMILY, DB_SUPPORTED_TRAITS_KEY,
};
use base::karma_coin::karma_coin_core_types::CharTrait::{Helpful, Kind, Smart};
use base::karma_coin::karma_coin_core_types::{TraitName, Traits};

//const GENESIS_FILE_NAME: &str = "genesis_config.toml";

impl DbConfigService {
    /// Config genesis static persistent data
    pub(crate) async fn config_genesis() -> Result<()> {
        info!("running genesis config...");

        let traits = Traits {
            // todo: traits should come from genesis config file
            // and not be handled with db at all
            named_traits: vec![
                TraitName::new(Kind, "Kind"),
                TraitName::new(Helpful, "Helpful"),
                TraitName::new(Smart, "Smart"),
            ],
        };

        use prost::Message;
        let mut buf = Vec::with_capacity(traits.encoded_len());
        if traits.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode default traits"));
        };

        // store default char traits
        // todo: move this to genesis config
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(DB_SUPPORTED_TRAITS_KEY.as_bytes()),
                value: Bytes::from(buf),
            },
            cf: BLOCKCHAIN_DATA_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // todo: load config from file and store in memory

        /*

        let builder = Config::builder();

        let _config = builder.add_source(config::File::with_name(GENESIS_FILE_NAME))
            .build()
            .unwrap();*/

        /*
            todo: initialize these settings - genesis config:

            uint32 network_id = 1;
            uint64 users_count = 2;
            uint64 genesis_time = 3;
            string name = 4;
            uint64 block_height = 5;
            string api_version = 6; // provided API semantic version
            uint64 transactions_count = 7; // number of transactions
            uint64 appreciations_count = 8; // number of appreciations
            uint64 new_account_reward = 9; // new account reward in kcents
            uint64 referral_reward = 10; // referral reward in kcents
        */

        Ok(())
    }
}
