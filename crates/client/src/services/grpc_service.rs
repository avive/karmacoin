// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use crate::simple_client::SimpleClient;
use anyhow::Result;
use bytes::Bytes;
use tonic::{Request, Response, Status};
use xactor::*;

/// SimpleClientGrpcService is a network service which provides a client grpc api
/// We use it to simulate user actions with a client for use cases such as setting service provider and
/// sending a text message to another client.
#[derive(Debug)]
pub(crate) struct ClientGrpcService {}

impl Default for ClientGrpcService {
    fn default() -> Self {
        debug!("SimpleClientGrpcService started");
        SimpleClientGrpcService {}
    }
}

impl ClientGrpcService {

}

/// SimpleClientGrpcService implements the SimpleClientService trait which defines the grpc
/// methods in the client's public api.
#[tonic::async_trait]
impl ClientApi for ClientGrpcService {

}
