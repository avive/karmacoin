// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::api::get_char_traits::GetCharTraits;
use crate::services::api::get_user_by_account_id::GetUserInfoByAccountId;
use crate::services::api::get_user_by_nick::GetUserInfoByNick;
use crate::services::api::get_user_by_number::GetUserInfoByNumber;
use crate::services::blockchain::block_event::GetBlocksEvents;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::mem_pool_service::{AddTransaction, MemPoolService};
use crate::services::blockchain::stats::GetStats;
use crate::services::blockchain::tx_event::GetTransactionEvents;
use crate::services::blockchain::txs_processor::ProcessTransactions;
use crate::services::blockchain::txs_store::{GetTransactionByHash, GetTransactionsByAccountId};
use anyhow::Result;
use base::karma_coin::karma_coin_api::api_service_server::ApiService as ApiServiceTrait;
use base::karma_coin::karma_coin_api::*;
use base::karma_coin::karma_coin_core_types::CharTrait;
use bytes::Bytes;
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct ApiService {
    pub(crate) char_traits: Option<Vec<CharTrait>>,
}

impl Default for ApiService {
    fn default() -> Self {
        info!("service created");
        ApiService { char_traits: None }
    }
}

#[async_trait::async_trait]
impl Actor for ApiService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("service started");
        Ok(())
    }
}

impl Service for ApiService {}

/// ApiService implements the ApiServiceTrait trait which defines the grpc rpc methods it provides for
/// clients over the network. All returned data is canonical blockchain data according to the state
/// of the backing blockchain node.
#[tonic::async_trait]
impl ApiServiceTrait for ApiService {
    /// Returns user info by nickname
    async fn get_user_info_by_nick(
        &self,
        request: Request<GetUserInfoByNickRequest>,
    ) -> Result<Response<GetUserInfoByNickResponse>, Status> {
        let service = ApiService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?;

        let res = service
            .call(GetUserInfoByNick(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(res))
    }

    /// Returns user info by verified mobile phone number
    async fn get_user_info_by_number(
        &self,
        request: Request<GetUserInfoByNumberRequest>,
    ) -> std::result::Result<Response<GetUserInfoByNumberResponse>, Status> {
        let service = ApiService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service
            .call(GetUserInfoByNumber(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(res))
    }

    /// Returns user info by his unique account id
    async fn get_user_info_by_account(
        &self,
        request: Request<GetUserInfoByAccountRequest>,
    ) -> Result<Response<GetUserInfoByAccountResponse>, Status> {
        let service = ApiService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?;

        let res = service
            .call(GetUserInfoByAccountId(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(res))
    }

    /// Returns supported phone verifiers identity (on-chain data)
    async fn get_phone_verifiers(
        &self,
        _request: Request<GetPhoneVerifiersRequest>,
    ) -> Result<Response<GetPhoneVerifiersResponse>, Status> {
        // todo: grab this from genesis config

        todo!()
    }

    /// Returns the supported character traits for this network
    async fn get_char_traits(
        &self,
        request: Request<GetCharTraitsRequest>,
    ) -> Result<Response<GetCharTraitsResponse>, Status> {
        let service = ApiService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?;

        let res = service
            .call(GetCharTraits(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(res))
    }

    /// Get current blockchain data
    async fn get_blockchain_data(
        &self,
        request: Request<GetBlockchainDataRequest>,
    ) -> Result<Response<GetBlockchainDataResponse>, Status> {
        // create a block with the transaction
        let service = BlockChainService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let resp = service
            .call(GetStats(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        Ok(Response::new(resp))
    }

    /// Returns genesis readonly data
    async fn get_genesis_data(
        &self,
        _request: Request<GetGenesisDataRequest>,
    ) -> Result<Response<GetGenesisDataResponse>, Status> {
        todo!()
    }

    /// Submit a transaction for processing to the mem pool
    async fn submit_transaction(
        &self,
        request: Request<SubmitTransactionRequest>,
    ) -> Result<Response<SubmitTransactionResponse>, Status> {
        let tx = request
            .into_inner()
            .transaction
            .ok_or_else(|| Status::invalid_argument("transaction is required"))?;

        let mem_pool = MemPoolService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("failed to get mempool: {}", e)))?;

        mem_pool
            .call(AddTransaction(tx))
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to process transaction: {}", e)))?;

        // start transaction processing to process all transactions in the mem pool
        // in production this can be done on a timer every few seconds
        // here we just trigger block production when a new transaction is submitted
        let service = BlockChainService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        service
            .call(ProcessTransactions {})
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?;

        Ok(Response::new(SubmitTransactionResponse {
            submit_transaction_result: SubmitTransactionResult::Submitted as i32,
        }))
    }

    /// Returns all transactions to, and or from an account
    async fn get_transactions(
        &self,
        request: Request<GetTransactionsRequest>,
    ) -> Result<Response<GetTransactionsResponse>, Status> {
        let account_id = request
            .into_inner()
            .account_id
            .ok_or_else(|| Status::invalid_argument("account id is required"))?;

        let service = BlockChainService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let txs = service
            .call(GetTransactionsByAccountId {
                account_id: Bytes::from(account_id.data),
            })
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?;

        // todo get all events for each transaction

        Ok(Response::new(GetTransactionsResponse {
            transactions: txs,
            tx_events: None,
        }))
    }

    /// Return transaction by its hash as well as stats (rejected, on-chain, mempool) and
    /// all known transaction events related to the transaction
    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status> {
        let service = BlockChainService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let tx_hash = Bytes::from(request.into_inner().tx_hash);

        let tx = service
            .call(GetTransactionByHash {
                hash: tx_hash.clone(),
            })
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?;

        let tx_events = service
            .call(GetTransactionEvents {
                tx_hash: tx_hash.clone(),
            })
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?;

        Ok(Response::new(GetTransactionResponse {
            transaction: tx,
            tx_events: Some(tx_events),
        }))
    }

    /// Returns blockchain events from a block height to a block height inclusive
    async fn get_blockchain_events(
        &self,
        request: Request<GetBlockchainEventsRequest>,
    ) -> Result<Response<GetBlockchainEventsResponse>, Status> {
        let service = BlockChainService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?;

        let req = request.into_inner();

        if req.from_block_height > req.to_block_height {
            return Err(Status::invalid_argument(
                "from block height must be less than or equal to to block height",
            ));
        }

        let res = service
            .call(GetBlocksEvents {
                from_height: req.from_block_height,
                to_height: req.to_block_height,
            })
            .await
            .map_err(|e| Status::internal(format!("internal error: {}", e)))?
            .map_err(|e| Status::internal(format!("failed to call blockchain api: {}", e)))?;

        Ok(Response::new(GetBlockchainEventsResponse {
            block_events: res,
        }))
    }
}
