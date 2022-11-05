use rocksdb::{ColumnFamilyDescriptor, Options};
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, ServerConfigService};
use db::db_service::DatabaseService;
use xactor::*;
use anyhow::Result;

/// cfs and data modeling
pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests
pub const VERIFIERS_COL_FAMILY: &str = "verifiers_cf"; // col family for verifiers data
pub const USERS_COL_FAMILY: &str = "users_cf"; // col family for user's data. index: accountId, data: User
pub const NICKS_COL_FAMILY: &str = "nicks_cf"; // col family for unique nicks (binencoded strings). data: accountId.

// tracking codes sent to new users before they are users
pub const VERIFICATION_CODES_COL_FAMILY: &str = "verification_codes_cf"; // index: verification_code, data: accountId. ttl: 24 hours
pub const MOBILE_NUMBERS_COL_FAMILY: &str = "mobile_number_cf"; // col family for unique numbers of registered users. data: accountId
pub const NET_SETTINGS_COL_FAMILY: &str = "net_settings_cf"; // col family for network settings.
pub const TRANSACTIONS_COL_FAMILY: &str = "txs_cf"; // signed transactions keyed by their hash
pub const BLOCKS_COL_FAMILY: &str = "blocks_cf"; // blocks keyed by block number - the blockchain


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
                ColumnFamilyDescriptor::new(TRANSACTIONS_COL_FAMILY, Options::default()),
            ],
        })
            .await?;

        Ok(())
    }
}

impl Service for DbConfigService {}
