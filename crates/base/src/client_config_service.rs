// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests

use crate::server_config_service::{
    DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, GRPC_SERVER_HOST_CONFIG_KEY,
    GRPC_SERVER_HOST_PORT_CONFIG_KEY,
};
use anyhow::{anyhow, Result};
use config::{Config, Environment};
use log::*;
use xactor::*;

pub const CLIENT_NAME_CONFIG_KEY: &str = "client_name";

#[derive(Debug, Clone, Default)]
pub struct ClientConfigService {
    config: Config,
    config_file: Option<String>,
}

#[async_trait::async_trait]
impl Actor for ClientConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Client ConfigService started");

        let mut config = Config::builder()
            .set_default(DROP_DB_CONFIG_KEY, true)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_PORT_CONFIG_KEY, 8081)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_CONFIG_KEY, "[::1]")
            .unwrap()
            .set_default(DB_NAME_CONFIG_KEY, "client_db")
            .unwrap()
            .set_default("client_name", "client_anon")
            .unwrap()
            .add_source(Environment::with_prefix("KC_CLIENT"))
            .build()
            .unwrap();

        // load configs from file if it was set
        if let Some(config_file) = &self.config_file {
            #[allow(deprecated)]
            config.merge(config::File::with_name(config_file)).unwrap();
        }

        self.config = config;

        Ok(())
    }
}

impl Service for ClientConfigService {}

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

#[async_trait::async_trait]
impl Handler<SetConfigFile> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        // todo: verify config file exists and is readable by this process
        #[allow(deprecated)]
        self.config
            .merge(config::File::with_name(msg.config_file.as_str()).required(false))
            .unwrap();

        self.config_file = Some(msg.config_file.clone());

        debug!(
            "Merged content of config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
    }
}

// helpers
impl ClientConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetValue(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = ClientConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }
}

#[message(result = "Option<bool>")]
pub struct GetBool(pub String);

/// Get bool value
#[async_trait::async_trait]
impl Handler<GetBool> for ClientConfigService {
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
impl Handler<GetU64> for ClientConfigService {
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
impl Handler<GetValue> for ClientConfigService {
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
impl Handler<SetValue> for ClientConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetValue) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
