// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use tonic::{Request, Response, Status};
use base::karma_coin::karma_coin_blockchain::{CreateBlockRequest, CreateBlockResponse,
                                              GetHeadHeightRequest,
                                              GetHeadHeightResponse,
                                              GetBlockByHeightRequest, GetBlockByHeightResponse,
                                              };
use xactor::*;

/// Blockchain service mocks a blockchain node
/// It provides a GRPC service defined in KarmaCoinBlockchainService
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
use crate::services::blockchain::create_block::CreateBlock;
use crate::services::blockchain::get_head_height::GetHeadHeight;

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



