// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;
use common::{create_user, finalize_test, init_test};

use base::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetUserInfoByAccountRequest, SubmitTransactionRequest, SubmitTransactionResult,
};
use base::karma_coin::karma_coin_core_types::TransactionType::PaymentV1;
use base::karma_coin::karma_coin_core_types::{
    AccountId, PaymentTransactionV1, SignedTransaction, TransactionBody, TransactionData,
};
use base::server_config_service::DEFAULT_GRPC_SERVER_PORT;
use chrono::Utc;
use prost::Message;
use server::server_service::{ServerService, Startup};
use xactor::Service;

/// Test payment transaction between 2 users
#[tokio::test(flavor = "multi_thread")]
async fn payment_tx_no_funds_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    // todo: figure out why grpc warmup is needed - without the delay we have random connection refused
    // from api client
    use tokio::time::{sleep, Duration};
    sleep(Duration::from_millis(300)).await;

    let (user1_key_pair, _, _) = create_user("avive".into(), "+972539805381".into())
        .await
        .unwrap();

    let (user2_key_pair, user2_number, _) = create_user("angel".into(), "+972549805382".into())
        .await
        .unwrap();

    let user1_account_id = AccountId {
        data: user1_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    let user2_account_id = AccountId {
        data: user2_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let user1 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user1_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    let user1_balance_pre = user1.balance;
    let payment_amount = user1_balance_pre + 1;

    let user2 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user2_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    let user2_balance_pre = user2.balance;

    // payment from user 1 to user 2
    let payment_tx = PaymentTransactionV1 {
        from: Some(user1_account_id.clone()),
        to_number: Some(user2_number.clone()),
        to_account_id: None,
        amount: payment_amount,
        char_trait_id: 1,
        community_id: 0,
    };

    let mut buf = Vec::with_capacity(payment_tx.encoded_len());
    payment_tx.encode(&mut buf).unwrap();

    let net_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
        .await
        .unwrap()
        .unwrap() as u32;

    let tx_body = TransactionBody {
        timestamp: Utc::now().timestamp_millis() as u64,
        nonce: 1,
        fee: 1,
        transaction_data: Some(TransactionData {
            transaction_data: buf,
            transaction_type: PaymentV1 as i32,
        }),
        net_id,
    };

    let mut buf1 = Vec::with_capacity(tx_body.encoded_len());
    tx_body.encode(&mut buf1).unwrap();

    let mut signed_tx = SignedTransaction {
        signer: Some(user1_account_id.clone()),
        transaction_body: buf1,
        signature: None,
    };

    signed_tx.signature = Some(signed_tx.sign(&user1_key_pair.to_ed2559_keypair()).unwrap());

    signed_tx.validate().await.expect("invalid transaction");

    let resp = api_client
        .submit_transaction(SubmitTransactionRequest {
            transaction: Some(signed_tx.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        resp.submit_transaction_result,
        SubmitTransactionResult::Submitted as i32,
    );

    // read updated user chain data
    let user1 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user1_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    // get user by account id
    let user2 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user2_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    // check no balances were changed (assumed tx_fee subsidy)
    assert_eq!(user1_balance_pre, user1.balance);
    assert_eq!(user2_balance_pre, user2.balance);

    finalize_test().await;
}
