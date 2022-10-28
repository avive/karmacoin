// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::fmt::Error;
use anyhow::Result;
use base::karma_coin::karma_coin_api::api_service_server::ApiService as ApiServiceTrait;
use tonic::{Code, IntoRequest, Request, Response, Status};
use base::karma_coin::karma_coin_api::{GetBlockchainEventsRequest, GetBlockchainEventsResponse, GetCharTraitsRequest, GetCharTraitsResponse, GetNetInfoRequest, GetNetInfoResponse, GetPhoneVerifiersRequest, GetPhoneVerifiersResponse, GetTransactionRequest, GetTransactionResponse, GetTransactionsRequest, GetTransactionsResponse, GetUserInfoByAccountRequest, GetUserInfoByAccountResponse, GetUserInfoByNumberRequest, GetUserInfoByNumberResponse, NicknameAvailableRequest, NicknameAvailableResponse, SubmitTransactionRequest, SubmitTransactionResponse};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct ApiService {}

impl Default for ApiService {
    fn default() -> Self {
        debug!("Api Service started");
        ApiService {}
    }
}

#[async_trait::async_trait]
impl Actor for ApiService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        debug!("ApiService started");
        Ok(())
    }
}

impl Service for ApiService {}

#[message(result = "Result<NicknameAvailableResponse>")]
pub(crate) struct GetNickNameAvailable(NicknameAvailableRequest);

#[async_trait::async_trait]
impl Handler<GetNickNameAvailable> for ApiService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetNickNameAvailable,
    ) -> Result<NicknameAvailableResponse> {
        unimplemented!()
    }
}


/// ApiService implements the ApiServiceTrait trait which defines the grpc rpc methods it provides for clients over the network
#[tonic::async_trait]
impl ApiServiceTrait for ApiService {
    async fn nickname_available(&self, request: Request<NicknameAvailableRequest>) -> Result<Response<NicknameAvailableResponse>, Status> {
        let service = ApiService::from_registry().await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service.call(GetNickNameAvailable(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    async fn submit_transaction(&self, request: Request<SubmitTransactionRequest>) -> std::result::Result<Response<SubmitTransactionResponse>, Status> {
        todo!()
    }

    async fn get_transactions_status(
        &self,
        request: Request<GetTransactionsRequest>,
    ) -> std::result::Result<Response<GetTransactionsResponse>, Status> {
        todo!()
    }

    async fn get_transactions(
        &self,
        request: Request<GetTransactionsRequest>,
    ) -> std::result::Result<Response<GetTransactionsResponse>, Status> {
        todo!()
    }

    async fn get_transaction(
        &self,
        request: Request<GetTransactionRequest>,
    ) -> std::result::Result<Response<GetTransactionResponse>, Status> {
        todo!()
    }

    async fn get_user_info_by_number(
        &self,
        request: Request<GetUserInfoByNumberRequest>,
    ) -> std::result::Result<Response<GetUserInfoByNumberResponse>, Status> {
        todo!()
    }

    async fn get_user_info_by_account(
        &self,
        request: Request<GetUserInfoByAccountRequest>,
    ) -> std::result::Result<Response<GetUserInfoByAccountResponse>, Status> {
        todo!()
    }

    async fn get_phone_verifiers(
        &self,
        request: Request<GetPhoneVerifiersRequest>,
    ) -> std::result::Result<Response<GetPhoneVerifiersResponse>, Status> {
        todo!()
    }

    async fn get_char_traits(
        &self,
        request: Request<GetCharTraitsRequest>,
    ) -> std::result::Result<Response<GetCharTraitsResponse>, Status> {
        todo!()
    }

    async fn get_net_info(
        &self,
        request: Request<GetNetInfoRequest>,
    ) -> std::result::Result<Response<GetNetInfoResponse>, Status> {
        todo!()
    }

    async fn get_blockchain_events(
        &self,
        request: Request<GetBlockchainEventsRequest>,
    ) -> std::result::Result<Response<GetBlockchainEventsResponse>, Status> {
        todo!()
    }
}
