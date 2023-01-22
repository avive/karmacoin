// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetBlockchainDataRequest, GetBlockchainEventsRequest, GetBlocksRequest, GetTransactionsRequest,
    GetUserInfoByAccountRequest, GetUserInfoByNumberRequest, GetUserInfoByUserNameRequest,
};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::{AccountId, MobileNumber};
use base::server_config_service::DEFAULT_GRPC_SERVER_PORT;
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
    let mobile_number = "+972549805381";

    //We use ipv4 for testing, we could also connect via [::1] or [::]
    let mut api_client =
        ApiServiceClient::connect(format!("http://127.0.0.1:{}", DEFAULT_GRPC_SERVER_PORT))
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
        .get_user_info_by_user_name(GetUserInfoByUserNameRequest {
            user_name: user_name.into(),
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

    // check data for newly created block

    let chain_stats = api_client
        .get_blockchain_data(GetBlockchainDataRequest {})
        .await
        .unwrap()
        .into_inner()
        .stats
        .unwrap();

    assert_eq!(chain_stats.tip_height, 1);
    assert_eq!(chain_stats.users_count, 1);
    assert_eq!(chain_stats.transactions_count, 1);
    assert_eq!(chain_stats.fee_subs_count, 1);

    // check block event

    let blocks_events = api_client
        .get_blockchain_events(GetBlockchainEventsRequest {
            from_block_height: 0,
            to_block_height: 1,
        })
        .await
        .unwrap()
        .into_inner()
        .blocks_events;

    assert_eq!(blocks_events.len(), 1, "expected 1 block event");
    let event = &blocks_events[0];
    assert_eq!(event.height, 1);

    let blocks = api_client
        .get_blocks(GetBlocksRequest {
            from_block_height: 0,
            to_block_height: 1,
        })
        .await
        .unwrap()
        .into_inner()
        .blocks;

    assert_eq!(blocks.len(), 1, "expected 1 block");
    let block = &blocks[0];
    assert_eq!(block.height, 1);
    assert_eq!(block.digest, event.block_hash);

    finalize_test().await;
}
