// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::server_config_service::DEFAULT_GRPC_SERVER_PORT;
use log::info;
/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::GetGenesisDataRequest;
use common::{finalize_test, init_test};

#[tokio::test(flavor = "multi_thread")]
async fn get_genesis_config() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    // todo: figure out why grpc warmup is needed - without the delay we have random connection refused
    // from api client
    use tokio::time::{sleep, Duration};
    sleep(Duration::from_millis(300)).await;

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let resp = api_client
        .get_genesis_data(GetGenesisDataRequest {})
        .await
        .unwrap()
        .into_inner();

    info!("genesis data: {:?}", resp);

    finalize_test().await;
}
