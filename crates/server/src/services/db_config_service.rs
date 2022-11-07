use rocksdb::{ColumnFamilyDescriptor, Options};
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, ServerConfigService};
use db::db_service::DatabaseService;
use xactor::*;
use anyhow::Result;

/// cfs and data modeling

////
// Verier local data
//////////////

// Tracking codes sent to new users before they are users
// index: verification_code, data: accountId. ttl: 24 hours
pub const VERIFICATION_CODES_COL_FAMILY: &str = "verification_codes_cf";

/////
//// Blockchain-based data indexes - indexing on-chain data
/////////////////

// col family the network settings
pub const NET_SETTINGS_COL_FAMILY: &str = "net_settings_cf";

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
        })
            .await?;

        Ok(())
    }
}

impl Service for DbConfigService {}
