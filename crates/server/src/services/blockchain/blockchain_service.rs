// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use tonic::{Request, Response, Status};
use base::blockchain_config_service::BlockchainConfigService;
use base::hex_utils::hex_from_string;
use base::karma_coin::karma_coin_blockchain::{CreateBlockRequest, CreateBlockResponse,
                                              GetHeadHeightRequest,
                                              GetHeadHeightResponse,
                                              GetBlockByHeightRequest, GetBlockByHeightResponse,
                                              };
use base::karma_coin::karma_coin_core_types::{KeyPair, PrivateKey, PublicKey};
use xactor::*;
use base::karma_coin::karma_coin_blockchain::blockchain_service_server::BlockchainService as BlockChainServiceTrait;
use crate::services::blockchain::block_creator::CreateBlock;
use crate::services::blockchain::get_head_height::GetHeadHeight;


// private identity key (ed25519)
pub const BLOCK_PRODUCER_ID_PRIVATE_KEY: &str = "block_producer_id_key_private";
pub const BLOCK_PRODUCER_ID_PUBLIC_KEY: &str = "block_producer_id_key_public";

/// Blockchain service mocks a blockchain node
/// It provides a GRPC service defined in KarmaCoinBlockchainService
/// It is a lower-level API than the KarmaCoin API - designed to be used internally in the server
#[derive(Debug, Clone)]
pub(crate) struct BlockChainService {
    pub(crate) id_key_pair : Option<KeyPair>
}

impl Default for BlockChainService {
    fn default() -> Self {
        info!("Blockchain created");
        BlockChainService {
            id_key_pair: None,
        }
    }
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
                        info!("loaded verifier id key pair from config")
                    },
                    None => {
                        panic!("invalid config: missing verifier id public key");
                    }
                }
            },
            None => {
                // no private key in config - generate new key pair
                self.id_key_pair = Some(KeyPair::new());
                info!("generated a new random verifier id key pair");
            }
        }

        Ok(())
    }
}

impl Service for BlockChainService {}

#[tonic::async_trait]
impl BlockChainServiceTrait for BlockChainService {
    async fn create_block(&self, request: Request<CreateBlockRequest>) -> Result<Response<CreateBlockResponse>, Status> {
        let service = BlockChainService::from_registry().await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service.call(CreateBlock(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {:?}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        Ok(Response::new(res))
    }

    async fn get_head_height(&self, request: Request<GetHeadHeightRequest>) -> Result<Response<GetHeadHeightResponse>, Status> {

        let service = BlockChainService::from_registry().await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service.call(GetHeadHeight(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {:?}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        Ok(Response::new(res))
    }

    async fn get_block_by_height(&self, _request: Request<GetBlockByHeightRequest>)
        -> Result<Response<GetBlockByHeightResponse>, Status>  {
        unimplemented!()
    }
}



