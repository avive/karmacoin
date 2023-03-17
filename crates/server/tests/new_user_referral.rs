// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate log;

#[path = "common/mod.rs"]
mod common;
use common::{create_user, finalize_test, init_test};

use base::genesis_config_service::{
    GenesisConfigService, AMBASSADOR_CHAR_TRAIT_ID, NET_ID_KEY, SIGNUP_CHAR_TRAIT_ID,
    SPENDER_CHAR_TRAIT_ID,
};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetTransactionsRequest, GetUserInfoByAccountRequest, SubmitTransactionRequest,
    SubmitTransactionResult,
};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::TransactionType::PaymentV1;
use base::karma_coin::karma_coin_core_types::{
    AccountId, BlockchainStats, MobileNumber, PaymentTransactionV1, SignedTransaction,
    TransactionBody, TransactionData,
};
use base::server_config_service::DEFAULT_GRPC_SERVER_PORT;
use chrono::Utc;
use log::info;
use prost::Message;

mod new_user_happy_flow;
mod payment_tx;

use server::server_service::{ServerService, Startup};
use server::Tokenomics;
use xactor::Service;

/// Basic referral flow test
#[tokio::test(flavor = "multi_thread")]
async fn referral_signup_happy_flow_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    use tokio::time::{sleep, Duration};
    sleep(Duration::from_millis(300)).await;

    // create user 1 - inviter
    let (user1_key_pair, _, _) = create_user("avive".into(), "+972539805381".into())
        .await
        .unwrap();

    let payment_amount = 10;

    // invited person mobile phone number
    let user2_phone_number = "+972549805382";

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let char_trait_id = 34;

    let user1_account_id = AccountId {
        data: user1_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    // Appreciation from user 1 to person 2 with phone number (not yet user 2)
    let payment_tx = PaymentTransactionV1 {
        from: Some(user1_account_id.clone()),
        to_number: Some(MobileNumber {
            number: user2_phone_number.into(),
        }),
        to_account_id: None,
        amount: payment_amount,
        char_trait_id,
        community_id: 0,
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
    info!("user balance pre referral: {}", user1_balance_pre);

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

    // user 2 signs up
    let (user2_key_pair, _, _) = create_user("rachel".into(), user2_phone_number.into())
        .await
        .unwrap();

    let user2_account_id = AccountId {
        data: user2_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    // get updated user 1 on chain data
    let user1 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user1_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    // get user 2 by account id onchain data
    let user2 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user2_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    let tokenomics = Tokenomics::new(BlockchainStats::new());
    let referral_reward = tokenomics.get_referral_reward_amount().await.unwrap();

    assert_eq!(
        user1_balance_pre + referral_reward - payment_amount,
        user1.balance,
        "unexpected payer balance"
    );

    // check that referral got the karma points rewards
    // 3 are expected - 1 for signup, 1 for referral and 1 for spender
    assert_eq!(user1.karma_score, 3);
    assert_eq!(user1.trait_scores.len(), 3);
    assert_eq!(user1.get_trait_score(SIGNUP_CHAR_TRAIT_ID, 0), 1);
    assert_eq!(user1.get_trait_score(AMBASSADOR_CHAR_TRAIT_ID, 0), 1);
    assert_eq!(user1.get_trait_score(SPENDER_CHAR_TRAIT_ID, 0), 1);

    // check appreciation stored in user2 account
    // 1 for signup, 1 for received appreciation
    assert_eq!(user2.karma_score, 2);
    assert_eq!(user2.trait_scores.len(), 2);
    assert_eq!(user2.get_trait_score(SIGNUP_CHAR_TRAIT_ID, 0), 1);
    assert_eq!(user2.get_trait_score(char_trait_id, 0), 1);

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
