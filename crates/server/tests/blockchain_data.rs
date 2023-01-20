// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::blockchain_config_service::DEFAULT_GRPC_SERVER_PORT;
use log::info;
/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::GetBlockchainDataRequest;
use common::{finalize_test, init_test};

#[tokio::test(flavor = "multi_thread")]
async fn get_blockchain_data() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let resp = api_client
        .get_blockchain_data(GetBlockchainDataRequest {})
        .await
        .unwrap()
        .into_inner();

    info!("blockchain data: {:?}", resp.stats.unwrap());

    finalize_test().await;
}
