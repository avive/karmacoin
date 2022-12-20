// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::blockchain_config_service::BlockchainConfigService;
use base::hex_utils::hex_from_string;
use base::karma_coin::karma_coin_core_types::{KeyPair, PrivateKey, PublicKey};
use xactor::*;


// private identity key (ed25519)
pub const BLOCK_PRODUCER_ID_PRIVATE_KEY: &str = "block_producer_id_key_private";
pub const BLOCK_PRODUCER_ID_PUBLIC_KEY: &str = "block_producer_id_key_public";

/// Blockchain service mocks a blockchain node
/// It provides a GRPC service defined in KarmaCoinBlockchainService
/// It is a lower-level API than the KarmaCoin API - designed to be used internally in the server
#[derive(Debug, Clone, Default)]
pub(crate) struct BlockChainService {
    pub(crate) id_key_pair : Option<KeyPair>
}

#[async_trait::async_trait]
impl Actor for BlockChainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("BlockChainService started");

        match BlockchainConfigService::get(BLOCK_PRODUCER_ID_PRIVATE_KEY.into())
            .await? {
            Some(key) => {
                // key is a hex string in config
                let private_key_data = hex_from_string(key).unwrap();

                match BlockchainConfigService::get(BLOCK_PRODUCER_ID_PUBLIC_KEY.into())
                    .await? {
                    Some(pub_key) => {
                        let pub_key_data = hex_from_string(pub_key).unwrap();
                        self.id_key_pair = Some(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey {
                                key: pub_key_data,
                            }),
                            scheme: 0
                        });
                        info!("loaded blockchain producer id key pair from config")
                    },
                    None => {
                        panic!("invalid config: missing blockchain producer id public key");
                    }
                }
            },
            None => {
                // no private key in config - generate new one
                self.id_key_pair = Some(KeyPair::new());
                info!("generated a new random block producer id key pair");
            }
        }

        Ok(())
    }
}

impl Service for BlockChainService {}





