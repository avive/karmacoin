// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::fmt::Error;
use anyhow::Result;

use base::karma_coin::karma_coin_verifier::phone_numbers_verifier_service_server::PhoneNumbersVerifierService;

use tonic::{Code, IntoRequest, Request, Response, Status};
use base::karma_coin::karma_coin_api::{GetBlockchainEventsRequest, GetBlockchainEventsResponse, GetCharTraitsRequest,
                                       GetCharTraitsResponse, GetNetInfoRequest, GetNetInfoResponse, GetPhoneVerifiersRequest,
                                       GetPhoneVerifiersResponse, GetTransactionRequest, GetTransactionResponse, GetTransactionsRequest,
                                       GetTransactionsResponse, GetUserInfoByAccountRequest, GetUserInfoByAccountResponse,
                                       GetUserInfoByNumberRequest, GetUserInfoByNumberResponse, SubmitTransactionRequest, SubmitTransactionResponse};
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, RegisterNumberResponse, SignUpUserRequest, SignUpUserResponse};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct VerifierService {}

impl Default for VerifierService {
    fn default() -> Self {
        info!("VerifierService created");
        VerifierService {}
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");
        Ok(())
    }
}

impl Service for VerifierService {}

/// ApiService implements the ApiServiceTrait trait which defines the grpc rpc methods it provides for clients over the network
#[tonic::async_trait]
impl PhoneNumbersVerifierService for VerifierService {
    async fn register_number(&self, request: Request<RegisterNumberRequest>) -> std::result::Result<Response<RegisterNumberResponse>, Status> {
        todo!()
    }

    async fn sign_up_user(&self, request: Request<SignUpUserRequest>) -> std::result::Result<Response<SignUpUserResponse>, Status> {
        todo!()
    }
}
