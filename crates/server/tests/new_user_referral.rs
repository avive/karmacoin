// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate log;

#[path = "common/mod.rs"]
mod common;
use common::{create_user, finalize_test, init_test};

use base::genesis_config_service::{GenesisConfigService, AMBASADOR_CHAR_TRAIT_ID, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetTransactionsRequest, GetUserInfoByAccountRequest, SubmitTransactionRequest,
    SubmitTransactionResult,
};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::TransactionType::PaymentV1;
use base::karma_coin::karma_coin_core_types::{
    AccountId, BlockchainStats, MobileNumber, PaymentTransactionV1, SignedTransaction,
    TransactionData,
};
use base::server_config_service::DEFAULT_GRPC_SERVER_PORT;
use base::signed_trait::SignedTrait;
use chrono::Utc;
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

    // create user 1 - inviter
    let (user1_key_pair, _, _) = create_user("avive".into(), "+972539805381".into())
        .await
        .unwrap();

    let payment_amount = 1;

    // invited person mobile phone number
    let user2_phone_number = "+972549805382";

    let mut api_client =
        ApiServiceClient::connect(format!("http://[::1]:{}", DEFAULT_GRPC_SERVER_PORT))
            .await
            .unwrap();

    let char_trait_id = 1;

    // Appreciation from user 1 to person 2 with phone number (not yet user 2)
    let payment_tx = PaymentTransactionV1 {
        to: Some(MobileNumber {
            number: user2_phone_number.into(),
        }),
        amount: payment_amount,
        char_trait_id,
    };

    let user1_account_id = AccountId {
        data: user1_key_pair.public_key.as_ref().unwrap().key.clone(),
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

    let mut buf = Vec::with_capacity(payment_tx.encoded_len());
    payment_tx.encode(&mut buf).unwrap();

    let net_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
        .await
        .unwrap()
        .unwrap() as u32;

    let mut signed_tx = SignedTransaction {
        signer: Some(user1_account_id.clone()),
        timestamp: Utc::now().timestamp_millis() as u64,
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

    // user 1 signs up

    // create user 2 - inviter
    let (user2_key_pair, _, _) = create_user("rachel".into(), user2_phone_number.into())
        .await
        .unwrap();

    let user2_account_id = AccountId {
        data: user2_key_pair.public_key.as_ref().unwrap().key.clone(),
    };

    // read updated user 1 chain data
    let user1 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user1_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    // get user 2 by account id
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
        user1.balance
    );

    // check that referral got the karma points rewards
    // 2 are expected - 1 for the referral and 1 for spender
    assert_eq!(user1.trait_scores.len(), 2);
    assert_eq!(
        user1.trait_scores[0].trait_id,
        AMBASADOR_CHAR_TRAIT_ID as u32
    );
    assert_eq!(user1.trait_scores[0].score, 1);

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
