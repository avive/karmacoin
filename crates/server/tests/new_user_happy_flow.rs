// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetTransactionsRequest, GetUserInfoByAccountRequest, GetUserInfoByNickRequest,
    GetUserInfoByNumberRequest,
};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::{AccountId, MobileNumber};
use common::{create_user, finalize_test, init_test};

/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

/// Test complete user signup flow
#[tokio::test(flavor = "multi_thread")]
async fn new_user_happy_flow_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();
    let user_name = "avive";
    let mobile_number = "972549805380";

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    let resp = create_user(user_name.into(), mobile_number.into())
        .await
        .unwrap();

    // verify user account on chain
    let account_id = resp.0.public_key.as_ref().unwrap().key.clone();

    // get user by account id
    let resp = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(AccountId {
                data: account_id.clone(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();

    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number
    );
    assert_eq!(resp_user.nonce, 1);

    // get user by name
    let resp = api_client
        .get_user_info_by_nick(GetUserInfoByNickRequest {
            nickname: user_name.into(),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();
    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number
    );
    assert_eq!(resp_user.nonce, 1);

    // get user by number
    let resp = api_client
        .get_user_info_by_number(GetUserInfoByNumberRequest {
            mobile_number: Some(MobileNumber {
                number: mobile_number.into(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();
    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number
    );
    assert_eq!(resp_user.nonce, 1);

    // verify that the new user transaction is on chain
    let resp = api_client
        .get_transactions(GetTransactionsRequest {
            account_id: Some(AccountId {
                data: account_id.clone(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(resp.transactions.len(), 1, "expected 1 transaction");
    let tx = &resp.transactions[0];
    assert_eq!(tx.status, OnChain as i32);

    assert!(resp.tx_events.is_some(), "expected tx events");
    assert_eq!(resp.tx_events.unwrap().events.len(), 1, "expected 1 event");

    finalize_test().await;
}
