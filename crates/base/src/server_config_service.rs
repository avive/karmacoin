// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::hex_utils::hex_from_string;
use crate::karma_coin::karma_coin_core_types::{KeyPair, PrivateKey, PublicKey};
use anyhow::{anyhow, Result};
use config::builder::DefaultState;
use config::{Config, ConfigBuilder, Environment};
use log::*;
use std::path::Path;
use xactor::*;

// Verifier data
pub const VERIFIER_NAME: &str = "KCV0.1";
pub const VERIFIER_ID_PRIVATE_KEY: &str = "verifier_id_key_private";
pub const VERIFIER_ID_PUBLIC_KEY: &str = "verifier_id_key_public";
pub const START_VERIFIER_SERVICE_CONFIG_KEY: &str = "start_verifier_service";

/// When true, verify will send invite sms messages
pub const SEND_INVITE_SMS_MESSAGES_CONFIG_KEY: &str = "send_sms_invites";
pub const MAX_SMS_INVITES_PER_NUMBER_CONFIG_KEY: &str = "max_sms_invites_per_number";
pub const SEND_INVITE_SMS_TASK_FREQ_SECS_CONFIG_KEY: &str = "send_sms_task_freq";
pub const SEND_INVITE_SMS_TIME_BETWEEN_SMS_SECS_CONFIG_KEY: &str = "send_sms_time_between";

pub const AUTH_SERVICE_HOST_KEY: &str = "auth_host_key";
pub const AUTH_SERVICE_PORT_KEY: &str = "auth_port_key";
pub const AUTH_SERVICE_PROTOCOL_KEY: &str = "auth_protocol_key";

pub const DEFAULT_GRPC_SERVER_PORT: i64 = 9080;
pub const DEFAULT_GRPC_ADMIN_PORT: i64 = 9888;
pub const DEFAULT_START_GRPC_SERVER: bool = true;

/// Start the verification service
pub const DEFAULT_START_VERIFIER_SERVICE: bool = true;
pub const DEFAULT_START_API_SERVICE: bool = true;
pub const DEFAULT_DROP_DB_ON_EXIT: bool = false;

/// ConfigService for servers

pub const DB_NAME_CONFIG_KEY: &str = "db_name";
pub const DROP_DB_CONFIG_KEY: &str = "drop_db_on_exit";
pub const SERVER_NAME_CONFIG_KEY: &str = "server_name";
pub const GRPC_SERVER_HOST_CONFIG_KEY: &str = "grpc_host"; //
pub const GRPC_SERVER_HOST_PORT_CONFIG_KEY: &str = "grpc_host_port";
pub const GRPC_ADMIN_PORT_CONFIG_KEY: &str = "grpc_admin_port";
pub const START_API_SERVICE_CONFIG_KEY: &str = "start_api_service";

pub const MEM_POOL_MAX_ITEMS_KEY: &str = "mem_pool_max_items_key";
pub const MEM_POOL_MAX_TX_AGE_HOURS: &str = "mem_pool_max_tx_age_key";

// private identity key (ed25519)
pub const BLOCK_PRODUCER_ID_PRIVATE_KEY: &str = "block_producer_id_key_private";
pub const BLOCK_PRODUCER_ID_PUBLIC_KEY: &str = "block_producer_id_key_public";
pub const BLOCK_PRODUCER_USER_NAME: &str = "block_producer_user_name";

pub struct ServerConfigService {
    config: Config,
    config_file: Option<String>,
}

impl ServerConfigService {
    fn get_default_builder(&self) -> ConfigBuilder<DefaultState> {
        Config::builder()
            .set_default(DROP_DB_CONFIG_KEY, DEFAULT_DROP_DB_ON_EXIT)
            .unwrap()
            .set_default(
                START_VERIFIER_SERVICE_CONFIG_KEY,
                DEFAULT_START_VERIFIER_SERVICE,
            )
            .unwrap()
            .set_default(START_API_SERVICE_CONFIG_KEY, DEFAULT_START_API_SERVICE)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_PORT_CONFIG_KEY, DEFAULT_GRPC_SERVER_PORT)
            .unwrap()
            .set_default(GRPC_ADMIN_PORT_CONFIG_KEY, DEFAULT_GRPC_ADMIN_PORT)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_CONFIG_KEY, "[::]")
            .unwrap()
            // we always want to have a peer name - even a generic one
            .set_default(SERVER_NAME_CONFIG_KEY, "KCBP0.1")
            .unwrap()
            .set_default(DB_NAME_CONFIG_KEY, "karmacoin_db")
            .unwrap()
            .set_default(MEM_POOL_MAX_ITEMS_KEY, 5000)
            .unwrap()
            .set_default(MEM_POOL_MAX_TX_AGE_HOURS, 168)
            .unwrap()
            .set_default(BLOCK_PRODUCER_USER_NAME, "Block producer 1")
            .unwrap()
            .set_default(BLOCK_PRODUCER_USER_NAME, "Block producer 1")
            .unwrap()
            // off by default to prevent charges - set to true to test sms messages
            .set_default(SEND_INVITE_SMS_MESSAGES_CONFIG_KEY, true)
            .unwrap()
            // how frequently to send sms
            .set_default(SEND_INVITE_SMS_TASK_FREQ_SECS_CONFIG_KEY, 30)
            .unwrap()
            .set_default(MAX_SMS_INVITES_PER_NUMBER_CONFIG_KEY, 2)
            .unwrap()
            // don't send invite sms more frequently then this cool down period
            .set_default(SEND_INVITE_SMS_TIME_BETWEEN_SMS_SECS_CONFIG_KEY, 3600)
            .unwrap()
            .set_default(
                BLOCK_PRODUCER_ID_PRIVATE_KEY,
                "67c31f3fb18572e97a851f757fc64fc1d0f8ed77c36abdd210f93711eb14f062",
            )
            .unwrap()
            .set_default(
                BLOCK_PRODUCER_ID_PUBLIC_KEY,
                "ec3d84d8e7ded4d438b67eae89ce3fb94c8d77fe0816af797fc40c9a6807a5cd",
            )
            .unwrap()
            // dev mode values
            .set_default(
                VERIFIER_ID_PRIVATE_KEY,
                "67c31f3fb18572e97a851f757fc64fc1d0f8ed77c36abdd210f93711eb14f062",
            )
            .unwrap()
            .set_default(
                VERIFIER_ID_PUBLIC_KEY,
                "ec3d84d8e7ded4d438b67eae89ce3fb94c8d77fe0816af797fc40c9a6807a5cd",
            )
            .unwrap()
            .set_default(AUTH_SERVICE_PORT_KEY, 8080)
            .unwrap()
            .set_default(AUTH_SERVICE_HOST_KEY, "127.0.0.1")
            .unwrap()
            .set_default(AUTH_SERVICE_PROTOCOL_KEY, "http")
            .unwrap()
            .add_source(
                Environment::with_prefix("KC")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
    }
}

#[async_trait::async_trait]
impl Actor for ServerConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        let mut builder = self.get_default_builder();

        // load configs from file if it was set
        if let Some(config_file) = &self.config_file {
            builder = builder.add_source(config::File::with_name(config_file));

            #[allow(deprecated)]
            self.config
                .merge(config::File::with_name(config_file))
                .unwrap();
        }

        // todo: if id private key not set then generate random keypair and store private key
        self.config = builder.build().unwrap();

        info!("service started");

        Ok(())
    }
}

impl Service for ServerConfigService {}

impl Default for ServerConfigService {
    fn default() -> Self {
        info!("Service created");
        ServerConfigService {
            config: Config::default(),
            config_file: None,
        }
    }
}

// helpers
impl ServerConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        //info!("Get config value for key: {}", key);
        let config = ServerConfigService::from_registry().await?;
        // info!("got service");
        let res = config.call(GetValue(key)).await?;
        // info!("got value for key: {:?}", res);
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

#[message(result = "Result<KeyPair>")]
pub struct GetVerifierIdKeyPair;

#[async_trait::async_trait]
impl Handler<GetVerifierIdKeyPair> for ServerConfigService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetVerifierIdKeyPair,
    ) -> Result<KeyPair> {
        match self.config.get_string(VERIFIER_ID_PRIVATE_KEY) {
            Ok(data) => {
                let private_key_data = hex_from_string(data).unwrap();
                match self.config.get_string(VERIFIER_ID_PUBLIC_KEY) {
                    Ok(pub_data) => {
                        let pub_key_data = hex_from_string(pub_data).unwrap();
                        Ok(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey { key: pub_key_data }),
                            scheme: 0,
                        })
                    }
                    Err(_) => {
                        panic!("invalid config file: missing verifier public key when private key is provided");
                    }
                }
            }
            Err(_) => {
                info!("no block producer private key in config - generating a new random verifier id key pair");
                Ok(KeyPair::new())
            }
        }
    }
}

#[message(result = "Result<KeyPair>")]
pub struct GetBlockProducerIdKeyPair;

#[async_trait::async_trait]
impl Handler<GetBlockProducerIdKeyPair> for ServerConfigService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetBlockProducerIdKeyPair,
    ) -> Result<KeyPair> {
        match self.config.get_string(BLOCK_PRODUCER_ID_PRIVATE_KEY) {
            Ok(data) => {
                let private_key_data = hex_from_string(data).unwrap();
                match self.config.get_string(BLOCK_PRODUCER_ID_PUBLIC_KEY) {
                    Ok(pub_data) => {
                        let pub_key_data = hex_from_string(pub_data).unwrap();
                        Ok(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey { key: pub_key_data }),
                            scheme: 0,
                        })
                    }
                    Err(_) => {
                        panic!("invalid config file: missing block producer public key when private key is provided");
                    }
                }
            }
            Err(_) => {
                info!("no block producer private key in config - generating a new random block producer id key pair");
                Ok(KeyPair::new())
            }
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

#[async_trait::async_trait]
impl Handler<SetConfigFile> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        info!("Current dir: {:?}", std::env::current_dir().unwrap());
        if !Path::new(&msg.config_file).exists() {
            warn!(
                "using default config. Requested config file {:?} does not exist",
                msg.config_file.as_str()
            );
            return Ok(());
        }

        let builder = self.get_default_builder();

        self.config = builder
            .add_source(config::File::with_name(&msg.config_file))
            .build()
            .unwrap();

        // save config file so it can be used if we need to reload config
        self.config_file = Some(msg.config_file.clone());

        info!(
            "merged content of server config file {:?}",
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
        // info!("Getting value for key {:?}", msg.0.as_str());
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
        #[allow(deprecated)]
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
        #[allow(deprecated)]
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
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
