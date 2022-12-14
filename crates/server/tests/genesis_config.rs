// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

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

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
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
