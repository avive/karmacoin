// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::server_config_service::{ServerConfigService, DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY};
use db::db_service::DatabaseService;
use rocksdb::{ColumnFamilyDescriptor, Options};
use xactor::*;

//////
// db data modeling - column families and their stored data
// Encoding conventions:
// - string keys are utf8 encoded to bytes
// - Numbers are LittleEndian encoded
// - Protobuf objects are serialized using prost
//////

////
// Verifier local data
//////////////

// Tracking codes sent to new users before they are users
// index: verification_code, data: accountId. ttl: 24 hours
pub const VERIFICATION_CODES_COL_FAMILY: &str = "verification_codes_cf";

// Unique reserved nicks (bin-coded strings). data: accountId. ttl: 24 hours
// Nicks are reserved by new users when they verify their phone so they can claim the nicks
// in up to 24 hours from verification via the CreateUser transaction
pub const RESERVED_NICKS_COL_FAMILY: &str = "reserved_nicks_cf";

/////
//// Blockchain-based data - indexing on-chain data and its blocks
/////////////////

// todo: add index of all block producers who got reward

/// col family for blockchain data. Various settings are accessible via keys.
pub const BLOCKCHAIN_DATA_COL_FAMILY: &str = "blockchain_data_cf";

/// value: chain aggregated data - number of blocks, number of transactions, etc.
pub const CHAIN_AGG_DATA_KEY: &str = "chain_agg_data_key";

/// Transactions processing events
/// key: tx_hash, value: zero or more tx events emitted by tx processing
pub const TRANSACTIONS_EVENTS_COL_FAMILY: &str = "txs_events_cf";

/// Block's transactions processing events
/// key: block height, value: zero or more events emitted by txs in the block
pub const BLOCK_EVENTS_COL_FAMILY: &str = "bc_events_cf";

/// col family for verifiers on-chain data. index: accountId, data: Verifier dial-up info
/// this data is in consensus on genesis and can only change via a runtime update
pub const VERIFIERS_COL_FAMILY: &str = "verifiers_cf";

/// A mapping of account ids to users. key: accountId, data: User
/// This is on-chain data.
/// All users accounts in consensus on-chain.
pub const USERS_COL_FAMILY: &str = "users_cf";

/// A mapping of nicknames to account ids.
/// This is on-chain data derived from on-chain users accounts data.
/// key: nickname (utf8 encoded string). value: accountId.
pub const NICKS_COL_FAMILY: &str = "nicks_cf";

/// A mapping from mobile phone numbers to registered users.
/// This is on-chain data derived from on-chain users accounts data.
/// key: mobile number (utf-8 encoded). value: accountId
pub const MOBILE_NUMBERS_COL_FAMILY: &str = "mobile_number_cf";

/// Signed transactions indexed by their hash. Data: SignTransaction
/// This is on-chain data
pub const TRANSACTIONS_COL_FAMILY: &str = "txs_cf";

// todo: add column family for transactions indexes - by signer and by payee for payment txs

/// Blocks keyed by block number - the blockchain. index: block height. value: Block
/// This is the actual blockchain
pub const BLOCKS_COL_FAMILY: &str = "blocks_cf";

/// Valid transactions submitted to the chain, not yet processed and queued in the txs pool
/// This is off-chain tx pool data
/// key: tx MEM_POOL. value: MemPool
///
pub const TXS_POOL_KEY: &str = "txs_pool_key";

pub const TXS_POOL_COL_FAMILY: &str = "txs_pool_cf";

/// Used for db testing - doesn't hold any app data
pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests

#[derive(Debug, Clone)]
pub(crate) struct DbConfigService {}

impl Default for DbConfigService {
    fn default() -> Self {
        info!("DbConfigService Service started");
        DbConfigService {}
    }
}

#[async_trait::async_trait]
impl Actor for DbConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Configuring db...");

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
                ColumnFamilyDescriptor::new(RESERVED_NICKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(VERIFIERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(USERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(NICKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(MOBILE_NUMBERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(VERIFICATION_CODES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BLOCKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BLOCK_EVENTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BLOCKCHAIN_DATA_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TXS_POOL_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TRANSACTIONS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TRANSACTIONS_EVENTS_COL_FAMILY, Options::default()),
            ],
        })
        .await
    }
}

impl Service for DbConfigService {}
