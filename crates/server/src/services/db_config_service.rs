use rocksdb::{ColumnFamilyDescriptor, Options};
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, ServerConfigService};
use db::db_service::DatabaseService;
use xactor::*;
use anyhow::Result;

pub const DB_NAME: &str = "karmacoin"; // todo: this must come from config
pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests

/// cfs
pub const VERIFIERS_COL_FAMILY: &str = "verifiers_cf"; // col family for verifiers data
pub const USERS_COL_FAMILY: &str = "users_cf"; // col family for user's data
pub const NICKS_COL_FAMILY: &str = "nicks_cf"; // col family for unique nicks

pub const NET_SETTINGS_COL_FAMILY: &str = "net_settings_cf"; // col family for network settings

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
