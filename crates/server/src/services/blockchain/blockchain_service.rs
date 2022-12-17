// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use tonic::{Request, Response, Status};
use base::karma_coin::karma_coin_blockchain::{AddBlockRequest, AddBlockResponse, GetBlockHeightRequest, GetBlockHeightResponse};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct BlockChainService {

}

impl Default for BlockChainService {
    fn default() -> Self {
        info!("Blockchain created");
        BlockChainService { }
    }
}

#[async_trait::async_trait]
impl Actor for BlockChainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("BlockChainService started");

        // todo: pull keys from config if they exist, otherwise generate new key pair for signing

        Ok(())
    }
}

impl Service for BlockChainService {}

use base::karma_coin::karma_coin_blockchain::blockchain_service_server::BlockchainService as BlockChainServiceTrait;

#[tonic::async_trait]
impl BlockChainServiceTrait for BlockChainService {
    async fn add_block(&self, _request: Request<AddBlockRequest>) -> Result<Response<AddBlockResponse>, Status> {
        todo!()
    }

    async fn get_block_height(&self, _request: Request<GetBlockHeightRequest>) -> Result<Response<GetBlockHeightResponse>, Status> {
        todo!()
    }
}




