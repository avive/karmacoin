// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use rocksdb::{ColumnFamilyDescriptor, Options};
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, ServerConfigService};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use base::karma_coin::karma_coin_core_types::CharTrait::{Helpful, Kind, Smart};
use base::karma_coin::karma_coin_core_types::{TraitName, Traits};

/// db data modeling - column families and their stored data
/// Encoding conventions:
/// - string keys are utf8 encoded to bytes
/// - Numbers are LittleEndian encoded
/// - Protobuf objects are serialized using prost

////
// Verier local data
//////////////

// Tracking codes sent to new users before they are users
// index: verification_code, data: accountId. ttl: 24 hours
pub const VERIFICATION_CODES_COL_FAMILY: &str = "verification_codes_cf";

// Unique reserved nicks (bin-coded strings). data: accountId. ttl: 24 hours
// Nicks are reserved by new users when they verify their phone so they can claim the nicks
// in up to 24 hours from verification via the CreateUser transaction
pub const RESERVED_NICKS_COL_FAMILY: &str = "reserbed_nicks_cf";


/////
//// Blockchain-based data - indexing on-chain data and the chain itself
/////////////////

// col family the network settings
pub const NET_SETTINGS_COL_FAMILY: &str = "net_settings_cf";

// key holds bool value indicating if db was initialized or needs initliaztion with
// static data
pub const DB_INITIALIZED_KEY: &str = "db_initialized_key";
pub const DB_SUPPORTED_TRAITS_KEY: &str = "supported_traits_key";

// col family for verifiers data. index: accountId, data: Verifier dial-up info
pub const VERIFIERS_COL_FAMILY: &str = "verifiers_cf";

// User's data. index: accountId, data: User
pub const USERS_COL_FAMILY: &str = "users_cf";

// Unique nicks (bin-coded strings). data: accountId.
pub const NICKS_COL_FAMILY: &str = "nicks_cf";

// Unique numbers of registered users. index: mobile number. data: accountId
pub const MOBILE_NUMBERS_COL_FAMILY: &str = "mobile_number_cf";

// signed transactions indexed by their hash. Data: SignTransaction
pub const TRANSACTIONS_COL_FAMILY: &str = "txs_cf";

// blocks keyed by block number - the blockchain. index: block height. Data: Block
pub const BLOCKS_COL_FAMILY: &str = "blocks_cf";

// valid transactions submitted to the chain and not yet processed. Queued in pool
pub const TXS_POOL_COL_FAMILY: &str = "txs_mem_pool_cf";

/////////////////////

pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests



#[derive(Debug, Clone)]
pub(crate) struct DbConfigService {
}

impl Default for DbConfigService {
    fn default() -> Self {
        info!("DbConfigService Service started");
        DbConfigService {}
    }
}

#[async_trait::async_trait]
impl Actor for DbConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Configuring the db...");

        let db_name = ServerConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let drop_on_exit = ServerConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        // configure the db
        DatabaseService::config_db(db::db_service::Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(VERIFIERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(USERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(NICKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(MOBILE_NUMBERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(VERIFICATION_CODES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(NET_SETTINGS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BLOCKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TXS_POOL_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TRANSACTIONS_COL_FAMILY, Options::default()),
            ],
        }).await?;

        // cehck if db was initialized with static net-specific data
        let init_key = Bytes::from(DB_INITIALIZED_KEY.as_bytes());
        if DatabaseService::read(ReadItem {
            key: init_key.clone(),
            cf: NET_SETTINGS_COL_FAMILY
        }).await?.is_none() {
            // initialize db static data here
            let traits = Traits {
                // todo: traits should come from config file
                named_traits: vec![
                    TraitName::new(Kind, "Kind"),
                    TraitName::new(Helpful, "Helpful"),
                    TraitName::new(Smart, "Smart"),
                ]
            };

            use prost::Message;
            let mut buf = Vec::with_capacity(traits.encoded_len());
            if traits.encode(&mut buf).is_err() {
                return Err(anyhow!("failed to encode default traits"));
            };

            // store default char traits
            DatabaseService::write(WriteItem {
                data: DataItem { key: Bytes::from(DB_SUPPORTED_TRAITS_KEY.as_bytes()),
                    value: Bytes::from(buf) },
                cf: NET_SETTINGS_COL_FAMILY,
                ttl: 0
            }).await?;
        }

        // mark that db is configured with static data
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: init_key,
                value: Bytes::from("1".as_bytes().to_vec()),
             },
            cf: NET_SETTINGS_COL_FAMILY,
            ttl: 0
        }).await?;

        Ok(())
    }
}

impl Service for DbConfigService {}
