// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use config::{Config, Environment};
use log::*;
use xactor::*;
use crate::hex_utils::hex_from_string;
use crate::karma_coin::karma_coin_core_types::{KeyPair, PrivateKey, PublicKey};

pub const DEFAULT_GRPC_SERVER_PORT: i64 = 9080;
pub const DEFAULT_GRPC_ADMIN_PORT: i64 = 9888;
pub const DEFAULT_START_GRPC_SERVER: bool = true;
pub const DEFAULT_START_VERIFIER_SERVICE: bool = true;
pub const DEFAULT_START_API_SERVICE: bool = true;
pub const DEFAULT_DROP_DB_ON_EXIT: bool = true;

/// ConfigService for servers

pub const DB_NAME_CONFIG_KEY: &str = "db_name";
pub const DROP_DB_CONFIG_KEY: &str = "drop_db_on_exit";
pub const SERVER_NAME_CONFIG_KEY: &str = "peer_name";
pub const GRPC_SERVER_HOST_CONFIG_KEY: &str = "grpc_host"; //
pub const GRPC_SERVER_HOST_PORT_CONFIG_KEY: &str = "grpc_admin_port";
pub const GRPC_ADMIN_PORT_CONFIG_KEY: &str = "grpc_admin_port";
pub const START_VERIFIER_SERVICE_CONFIG_KEY: &str = "start_verifier_service";
pub const START_API_SERVICE_CONFIG_KEY: &str = "start_api_service";

pub const MEM_POOL_MAX_ITEMS_KEY: &str = "mem_pool_max_items_key";

// todo: add verifier name
pub const VERIFIER_NAME: &str = "KarmaCoinVerifier_v1";

// private identity key (ed25519)
pub const BLOCK_PRODUCER_ID_PRIVATE_KEY: &str = "block_producer_id_key_private";
pub const BLOCK_PRODUCER_ID_PUBLIC_KEY: &str = "block_producer_id_key_public";
pub const BLOCK_PRODUCER_USER_NAME: &str = "block_producer_user_name";

// private identity key (ed25519)
pub const VERIFIER_ID_PRIVATE_KEY: &str = "id_verifier_key_private";
pub const VERIFIER_ID_PUBLIC_KEY: &str = "id_verifier_key_public";

pub struct ServerConfigService {
    config: Config,
    config_file: Option<String>
}

#[async_trait::async_trait]
impl Actor for ServerConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("server ConfigService started");

        let config = Config::builder()
            .set_default(DROP_DB_CONFIG_KEY, DEFAULT_DROP_DB_ON_EXIT)
            .unwrap()
            .set_default(START_VERIFIER_SERVICE_CONFIG_KEY, DEFAULT_START_VERIFIER_SERVICE)
            .unwrap()
            .set_default(
                START_API_SERVICE_CONFIG_KEY,
                DEFAULT_START_API_SERVICE,
            )
            .unwrap()
            .set_default(GRPC_SERVER_HOST_PORT_CONFIG_KEY, DEFAULT_GRPC_SERVER_PORT)
            .unwrap()
            .set_default(GRPC_ADMIN_PORT_CONFIG_KEY, DEFAULT_GRPC_ADMIN_PORT)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_CONFIG_KEY, "[::1]")
            .unwrap()
            // we always want to have a peer name - even a generic one
            .set_default(SERVER_NAME_CONFIG_KEY, "kc_verifier_1")
            .unwrap()
            .set_default(DB_NAME_CONFIG_KEY, "karmacoin_db")
            .unwrap()

            .set_default(MEM_POOL_MAX_ITEMS_KEY, 5000)
            .unwrap()

            .set_default(BLOCK_PRODUCER_USER_NAME, "a block producer")
            .unwrap()

            .add_source(
                Environment::with_prefix("KARMACOIN")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            ).build().unwrap();

        // load configs from file if it was set
        if let Some(config_file) = &self.config_file {
            #[allow(deprecated)]
            self.config.merge(config::File::with_name(config_file)).unwrap();
        }

        // todo: if id private key not set then generate random keypair and store private key
        self.config = config;

        Ok(())
    }
}

impl Service for ServerConfigService {}

impl Default for ServerConfigService {
    fn default() -> Self {
        info!("Configuring server...");
        ServerConfigService {
            config: Config::default(),
            config_file: None,
        }
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


#[message(result = "Result<KeyPair>")]
pub struct GetBlockProducerIdKeyPair;

#[async_trait::async_trait]
impl Handler<GetBlockProducerIdKeyPair> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: GetBlockProducerIdKeyPair) -> Result<KeyPair> {
        match ServerConfigService::get(BLOCK_PRODUCER_ID_PRIVATE_KEY.into())
            .await? {
            Some(key) => {
                // key is a hex string in config
                let private_key_data = hex_from_string(key).unwrap();

                match ServerConfigService::get(BLOCK_PRODUCER_ID_PUBLIC_KEY.into())
                    .await? {
                    Some(pub_key) => {
                        let pub_key_data = hex_from_string(pub_key).unwrap();
                        info!("loaded blockchain producer id key pair from config");
                        Ok(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey {
                                key: pub_key_data,
                            }),
                            scheme: 0
                        })
                    },
                    None => {
                        panic!("invalid config file: missing block producer id public key");
                    }
                }
            },
            None => {
                // no private key in config - generate new one
                info!("generated a new random block producer id key pair");
                Ok(KeyPair::new())

                // todo: store keypair in config so it is obtained on next call (after blockchain service restart for example)
            }
        }
    }
}


#[message(result = "Result<KeyPair>")]
pub struct GetVerifierKeyPair;

#[async_trait::async_trait]
impl Handler<GetVerifierKeyPair> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: GetVerifierKeyPair) -> Result<KeyPair> {
        match ServerConfigService::get(VERIFIER_ID_PRIVATE_KEY.into())
            .await? {
            Some(key) => {
                // key is a hex string in config
                let private_key_data = hex_from_string(key).unwrap();

                match ServerConfigService::get(VERIFIER_ID_PUBLIC_KEY.into())
                    .await? {
                    Some(pub_key) => {
                        let pub_key_data = hex_from_string(pub_key).unwrap();
                        info!("loaded blockchain verifier id key pair from config");
                        Ok(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey {
                                key: pub_key_data,
                            }),
                            scheme: 0
                        })
                    },
                    None => {
                        panic!("invalid config file: missing verifier id public key");
                    }
                }
            },
            None => {
                // no private key in config - generate new one
                info!("generated a new random block producer id key pair");
                Ok(KeyPair::new())

                // todo: store keypair in config so it is obtained on next call
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

        // todo: verify config file exists and is readable by this process

        #[allow(deprecated)]
        self.config
            .merge(config::File::with_name(&msg.config_file))
            .unwrap();

        // save config file so it can be used if we need to reload config
        self.config_file = Some(msg.config_file.clone());

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
