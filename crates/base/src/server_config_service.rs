// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use config::{Config, Environment};
use log::*;
use xactor::*;

pub const DEFAULT_GRPC_SERVER_PORT: i64 = 9080;
pub const DEFAULT_GRPC_ADMIN_PORT: i64 = 9888;
pub const DEFAULT_START_ADMIN_SERVICE: bool = true;
pub const DEFAULT_START_GRPC_SERVICE: bool = true;
pub const DEFAULT_DROP_DB_ON_EXIT: bool = true;

/// ConfigService for servers

pub const DB_NAME_CONFIG_KEY: &str = "db_name";
pub const DROP_DB_CONFIG_KEY: &str = "drop_db_on_exit";
pub const PEER_NAME_CONFIG_KEY: &str = "peer_name";
pub const GRPC_HOST_CONFIG_KEY: &str = "grpc_host"; // grpc api service host
pub const GRPC_SERVER_PORT_CONFIG_KEY: &str = "grpc_server_port"; // grpc api service port
pub const NET_ID_CONFIG_KEY: &str = "net_id";
pub const GRPC_ADMIN_PORT_CONFIG_KEY: &str = "grpc_admin_port";
pub const START_GRPC_SERVER_ADMIN_SERVICE_CONFIG_KEY: &str = "start_grpc_admin_service";
pub const START_GRPC_SERVICE_CONFIG_KEY: &str = "start_grpc_service";

pub struct ServerConfigService {
    config: Config,
}

#[async_trait::async_trait]
impl Actor for ServerConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("server ConfigService started");
        Ok(())
    }
}

impl Service for ServerConfigService {}

impl Default for ServerConfigService {
    fn default() -> Self {
        let mut config = Config::default();

        // todo: set default blockchain server config for server

        config
            .set_default(NET_ID_CONFIG_KEY, 0)
            .unwrap()
            .set_default(DROP_DB_CONFIG_KEY, DEFAULT_DROP_DB_ON_EXIT)
            .unwrap()
            .set_default(START_GRPC_SERVICE_CONFIG_KEY, DEFAULT_START_GRPC_SERVICE)
            .unwrap()
            .set_default(
                START_GRPC_SERVER_ADMIN_SERVICE_CONFIG_KEY,
                DEFAULT_START_ADMIN_SERVICE,
            )
            .unwrap()
            .set_default(GRPC_SERVER_PORT_CONFIG_KEY, DEFAULT_GRPC_SERVER_PORT)
            .unwrap()
            .set_default(GRPC_ADMIN_PORT_CONFIG_KEY, DEFAULT_GRPC_ADMIN_PORT)
            .unwrap()
            .set_default(GRPC_HOST_CONFIG_KEY, "[::1]")
            .unwrap()
            // we always want to have a peer name - even a generic one
            .set_default(PEER_NAME_CONFIG_KEY, "my_peer")
            .unwrap()
            .set_default(DB_NAME_CONFIG_KEY, "upsetter_db")
            .unwrap()
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(Environment::with_prefix("UPSETTER"))
            .unwrap();

        ServerConfigService { config }
    }
}

// helpers
impl ServerConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        let config = ServerConfigService::from_registry().await?;
        let res = config.call(GetValue(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = ServerConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = ServerConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }

    pub async fn set(key: String, value: String) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetValue { key, value }).await?
    }

    // helper
    pub async fn set_bool(key: String, value: bool) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetBool { key, value }).await?
    }

    // helper
    pub async fn set_u64(key: String, value: u64) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetU64 { key, value }).await?
    }
}

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

#[async_trait::async_trait]
impl Handler<SetConfigFile> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        // todo: verify config file exists and is readable by this process
        self.config
            .merge(config::File::with_name(msg.config_file.as_str()).required(false))
            .unwrap();

        debug!(
            "Merging content of config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
    }
}

#[message(result = "Option<bool>")]
pub struct GetBool(pub String);

#[async_trait::async_trait]
impl Handler<GetBool> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetBool) -> Option<bool> {
        match self.config.get_bool(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<u64>")]
pub struct GetU64(pub String);

#[async_trait::async_trait]
impl Handler<GetU64> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetU64) -> Option<u64> {
        match self.config.get_int(&msg.0.as_str()) {
            Ok(res) => Some(res as u64),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<String>")]
pub struct GetValue(pub String);

#[async_trait::async_trait]
impl Handler<GetValue> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetValue) -> Option<String> {
        match self.config.get_string(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetValue {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
impl Handler<SetValue> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetValue) -> Result<()> {
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetU64 {
    pub key: String,
    pub value: u64,
}

#[async_trait::async_trait]
impl Handler<SetU64> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetU64) -> Result<()> {
        match self.config.set(msg.key.as_str(), msg.value.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetBool {
    pub key: String,
    pub value: bool,
}

#[async_trait::async_trait]
impl Handler<SetBool> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetBool) -> Result<()> {
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
