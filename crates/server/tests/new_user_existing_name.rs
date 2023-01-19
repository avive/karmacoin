// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::GetUserInfoByNumberRequest;
use base::karma_coin::karma_coin_core_types::MobileNumber;
use common::{create_user, finalize_test, init_test};

/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

// Attempt to create new user with an existing username
#[tokio::test(flavor = "multi_thread")]
async fn new_user_existing_user_name() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    create_user("avive".into(), "+972539805381".into())
        .await
        .unwrap();

    create_user("avive".into(), "+972549805382".into())
        .await
        .unwrap();

    let mut api_client = ApiServiceClient::connect("http://[::1]:9080")
        .await
        .unwrap();

    let response = api_client
        .get_user_info_by_number(GetUserInfoByNumberRequest {
            mobile_number: Some(MobileNumber {
                number: "+972539805381".into(),
            }),
        })
        .await
        .unwrap();

    assert!(response.into_inner().user.is_some());

    let response = api_client
        .get_user_info_by_number(GetUserInfoByNumberRequest {
            mobile_number: Some(MobileNumber {
                number: "+972549805382".into(),
            }),
        })
        .await
        .unwrap();

    assert!(response.into_inner().user.is_none());

    finalize_test().await;
}
