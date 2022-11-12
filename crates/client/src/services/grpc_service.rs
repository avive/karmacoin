// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use tonic::{Request, Response, Status};
use base::karma_coin::karma_coin_client::client_api_server::ClientApi;
use base::karma_coin::karma_coin_client::{ConfigureRequest, ConfigureResponse, GetAccountStateRequest, GetAccountStateResponse, SendCoinRequest, SendCoinResponse, SignUpRequest, SignUpResponse, UpdateUserInfoRequest, UpdateUserInfoResponse};

/// SimpleClientGrpcService is a network service which provides a client grpc api
/// We use it to simulate user actions with a client for use cases such as setting service provider and
/// sending a text message to another client.
#[derive(Debug)]
pub(crate) struct ClientGrpcService {}

impl Default for ClientGrpcService {
    fn default() -> Self {
        debug!("SimpleClientGrpcService started");
        ClientGrpcService {}
    }
}

impl ClientGrpcService {

}

/// SimpleClientGrpcService implements the SimpleClientService trait which defines the grpc
/// methods in the client's public api.
#[tonic::async_trait]
impl ClientApi for ClientGrpcService {
    async fn configure(&self, request: Request<ConfigureRequest>) -> std::result::Result<Response<ConfigureResponse>, Status> {
        todo!()
    }

    async fn sign_up(&self, request: Request<SignUpRequest>) -> std::result::Result<Response<SignUpResponse>, Status> {
        todo!()
    }

    async fn update_user_info(&self, request: Request<UpdateUserInfoRequest>) -> std::result::Result<Response<UpdateUserInfoResponse>, Status> {
        todo!()
    }

    async fn send_coin(&self, request: Request<SendCoinRequest>) -> std::result::Result<Response<SendCoinResponse>, Status> {
        todo!()
    }

    async fn get_account_data(&self, request: Request<GetAccountStateRequest>) -> std::result::Result<Response<GetAccountStateResponse>, Status> {
        todo!()
    }
}
