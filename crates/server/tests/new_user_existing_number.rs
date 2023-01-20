// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::blockchain_config_service::DEFAULT_GRPC_SERVER_PORT;
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{GetUserInfoByNumberRequest, GetUserInfoByUserNameRequest};
use base::karma_coin::karma_coin_core_types::MobileNumber;
use common::{create_user, finalize_test, init_test};

/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

// Attempt to create new user with an existing number
#[tokio::test(flavor = "multi_thread")]
async fn new_user_existing_number() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    let res1 = create_user("avive".into(), "+972549805381".into())
        .await
        .unwrap();

    let account_id1 = res1.0.public_key.unwrap().key;

    // tx should be submitted ok but it should generate a tx event with an error message
    let res2 = create_user("angel".into(), "+972549805381".into())
        .await
        .unwrap();

    let account_id2 = res2.0.public_key.unwrap().key;

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let response = api_client
        .get_user_info_by_number(GetUserInfoByNumberRequest {
            mobile_number: Some(MobileNumber {
                number: "+972549805381".into(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    assert!(response.user.is_some(), "expected user to exist");
    assert_eq!(
        response.user.as_ref().unwrap().user_name,
        "angel",
        "expected angel's account"
    );
    assert_eq!(
        response.user.unwrap().account_id.unwrap().data,
        account_id2,
        "expected angel's account"
    );

    let response = api_client
        .get_user_info_by_user_name(GetUserInfoByUserNameRequest {
            user_name: "avive".to_string(),
        })
        .await
        .unwrap()
        .into_inner();

    assert!(
        response.user.is_some(),
        "this account should still exist on-chain"
    );
    assert_eq!(
        response.user.unwrap().account_id.unwrap().data,
        account_id1,
        "expected avivs's account"
    );

    finalize_test().await;
}
