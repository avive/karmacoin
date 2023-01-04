// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;
use common::{create_user, finalize_test, init_test};

use base::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetTransactionsRequest, GetUserInfoByAccountRequest, SubmitTransactionRequest,
    SubmitTransactionResult,
};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::TransactionType::PaymentV1;
use base::karma_coin::karma_coin_core_types::{
    AccountId, PaymentTransactionV1, SignedTransaction, TransactionData,
};
use base::signed_trait::SignedTrait;
use chrono::Utc;
use prost::Message;
use server::server_service::{ServerService, Startup};
use xactor::Service;

/// Test payment transaction between 2 users
#[tokio::test(flavor = "multi_thread")]
async fn payment_tx_happy_flow() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    let (user1_key_pair, _) = create_user("avive".into(), "972549805380".into())
        .await
        .unwrap();

    let (user2_key_pair, user2_number) = create_user("angel".into(), "972549805381".into())
        .await
        .unwrap();

    let payment_amount = 100;

    //
    let char_trait_id = 1;

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    // payment from user 1 to user 2
    let payment_tx = PaymentTransactionV1 {
        to: Some(user2_number.clone()),
        amount: payment_amount,
        char_trait_id,
    };

    let user1_account_id = AccountId {
        data: user1_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    let user2_account_id = AccountId {
        data: user2_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

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

    let user2_balance_pre = user2.balance;

    let mut buf = Vec::with_capacity(payment_tx.encoded_len());
    payment_tx.encode(&mut buf).unwrap();

    let net_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
        .await
        .unwrap()
        .unwrap() as u32;

    let mut signed_tx = SignedTransaction {
        signer: Some(user1_account_id.clone()),
        timestamp: Utc::now().timestamp_nanos() as u64,
        nonce: 2,
        fee: 10,
        transaction_data: Some(TransactionData {
            transaction_data: buf,
            transaction_type: PaymentV1 as i32,
        }),
        net_id,
        signature: None,
    };

    signed_tx.signature = Some(signed_tx.sign(&user1_key_pair.to_ed2559_keypair()).unwrap());

    signed_tx.validate(1).await.expect("invalid transaction");

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

    assert_eq!(user1_balance_pre - payment_amount, user1.balance);
    assert_eq!(user2_balance_pre + payment_amount, user2.balance);

    // check appreciation stored in user2 account
    assert_eq!(user2.trait_scores.len(), 1);
    assert_eq!(user2.trait_scores[0].trait_id, char_trait_id);
    assert_eq!(user2.trait_scores[0].score, 1);

    // verify that the payment transaction is on chain indexed by user 1
    let resp = api_client
        .get_transactions(GetTransactionsRequest {
            account_id: Some(AccountId {
                data: user1_account_id.data.clone(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        resp.transactions.len(),
        2,
        "user1 should have 2 transactions"
    );

    // check that the payment transaction is on chain
    let tx = &resp.transactions[1];
    assert_eq!(tx.status, OnChain as i32);

    // verify that the payment transaction is on chain indexed by user 2
    let resp = api_client
        .get_transactions(GetTransactionsRequest {
            account_id: Some(AccountId {
                data: user2_account_id.data.clone(),
            }),
        })
        .await
        .unwrap()
        .into_inner();

    assert_eq!(
        resp.transactions.len(),
        2,
        "user2 should have 2 transactions"
    );

    // check that the payment transaction is on chain
    let tx = &resp.transactions[1];
    assert_eq!(tx.status, OnChain as i32);

    finalize_test().await;
}
