// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

use base::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{
    GetUserInfoByAccountRequest, GetUserInfoByNickRequest, GetUserInfoByNumberRequest,
    SubmitTransactionRequest, SubmitTransactionResult,
};
use base::karma_coin::karma_coin_core_types::TransactionType::{NewUserV1, PaymentV1};
use base::karma_coin::karma_coin_core_types::VerifyNumberResult::Verified;
use base::karma_coin::karma_coin_core_types::{
    AccountId, BlockchainStats, CharTrait, KeyPair, MobileNumber, PaymentTransactionV1,
};
use base::karma_coin::karma_coin_core_types::{
    NewUserTransactionV1, SignedTransaction, TransactionData,
};
use base::karma_coin::karma_coin_verifier::verifier_service_client::VerifierServiceClient;
use base::karma_coin::karma_coin_verifier::RegisterNumberResult::CodeSent;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, VerifyNumberRequest};
use base::signed_trait::SignedTrait;
use base::test_helpers::enable_logger;
use chrono::Utc;
use db::db_service::DatabaseService;
use prost::Message;
use server::server_service::{ServerService, Startup};
use server::Tokenomics;
use xactor::*;

// helper function to create a new user
async fn create_user(user_name: String, number: String) -> Result<(KeyPair, MobileNumber)> {
    let user_key_pair = KeyPair::new();
    let user_ed_key_pair = user_key_pair.to_ed2559_keypair();
    let account_id_bytes = user_ed_key_pair.public.to_bytes().to_vec();

    let mobile_number = MobileNumber { number };
    let account_id = AccountId {
        data: account_id_bytes.clone(),
    };

    let mut register_number_request = RegisterNumberRequest::new();
    register_number_request.mobile_number = Some(mobile_number.clone());
    register_number_request.account_id = Some(account_id.clone());
    register_number_request.signature =
        Some(register_number_request.sign(&user_ed_key_pair).unwrap());

    register_number_request
        .verify_signature()
        .expect("signature should be valid");

    let mut verifier_service_client = VerifierServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    let resp = verifier_service_client
        .register_number(register_number_request)
        .await
        .unwrap()
        .into_inner();

    assert_eq!(resp.result, CodeSent as i32);

    info!("number registered");

    // obtain the verification code from the result as there's no sms service yet
    let code = resp.code;

    let mut v_request = VerifyNumberRequest::new();
    v_request.mobile_number = Some(mobile_number.clone());
    v_request.account_id = Some(account_id.clone());

    // in production this code is obtained from the sms message sent by verifier
    v_request.code = code;

    // user's requested nickname
    v_request.nickname = user_name.clone();
    v_request.signature = Some(v_request.sign(&user_ed_key_pair).unwrap());
    v_request
        .verify_signature()
        .expect("signature verification failed");

    let resp1 = verifier_service_client
        .verify_number(v_request)
        .await
        .unwrap();

    let v_resp = resp1.into_inner();
    assert_eq!(v_resp.result, Verified as i32);

    v_resp
        .verify_signature()
        .expect("invalid evidence signature");

    info!("verify evidence verified");

    let new_user_tx = NewUserTransactionV1 {
        verify_number_response: Some(v_resp.clone()),
    };

    let mut buf = Vec::with_capacity(new_user_tx.encoded_len());
    new_user_tx.encode(&mut buf).unwrap();

    let net_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
        .await
        .unwrap()
        .unwrap() as u32;

    let mut signed_tx = SignedTransaction {
        signer: Some(account_id.clone()),
        timestamp: Utc::now().timestamp_nanos() as u64,
        nonce: 1,
        fee: 10,
        transaction_data: Some(TransactionData {
            transaction_data: buf,
            transaction_type: NewUserV1 as i32,
        }),
        net_id,
        signature: None,
    };

    signed_tx.signature = Some(signed_tx.sign(&user_ed_key_pair).unwrap());

    signed_tx.validate(0).await.expect("invalid transaction");
    info!("new user tx signature's valid");

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

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

    // verify user account on chain

    // get user by account id
    let resp = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();

    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number.number
    );
    assert_eq!(resp_user.nonce, 1);

    // get user by name
    let resp = api_client
        .get_user_info_by_nick(GetUserInfoByNickRequest {
            nickname: user_name.clone(),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();
    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number.number
    );
    assert_eq!(resp_user.nonce, 1);

    // get user by number
    let resp = api_client
        .get_user_info_by_number(GetUserInfoByNumberRequest {
            mobile_number: Some(mobile_number.clone()),
        })
        .await
        .unwrap()
        .into_inner();

    let resp_user = resp.user.as_ref().unwrap();
    assert_eq!(resp_user.user_name, user_name);
    assert_eq!(
        resp_user.mobile_number.as_ref().unwrap().number,
        mobile_number.number
    );
    assert_eq!(resp_user.nonce, 1);

    Ok((user_key_pair, mobile_number))
}

/// tests in this file should be run sequentially and not in parallel

/// Test complete user signup flow
#[tokio::test(flavor = "multi_thread")]
async fn new_user_happy_flow_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    create_user("avive".into(), "972549805380".into())
        .await
        .unwrap();

    create_user("angel".into(), "972549805381".into())
        .await
        .unwrap();

    finalize_test().await;
}

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

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    // payment from user 1 to user 2
    let payment_tx = PaymentTransactionV1 {
        to: Some(user2_number.clone()),
        amount: payment_amount,
        char_trait: CharTrait::Kind as i32,
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

    finalize_test().await;
}

/// Basic referral flow test
#[tokio::test(flavor = "multi_thread")]
async fn referral_signup_happy_flow_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    // create user 1 - inviter
    let (user1_key_pair, _) = create_user("avive".into(), "972549805380".into())
        .await
        .unwrap();

    let payment_amount = 1;

    // invited person mobile phone number
    let user2_phone_number = "972549805381";

    let mut api_client = ApiServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    // Appreciation from user 1 to person 2 with phone number (not yet user 2)
    let payment_tx = PaymentTransactionV1 {
        to: Some(MobileNumber {
            number: user2_phone_number.into(),
        }),
        amount: payment_amount,
        char_trait: CharTrait::Kind as i32,
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

    // user 1 signs up

    // create user 2 - inviter
    let (user2_key_pair, _) = create_user("rachel".into(), user2_phone_number.into())
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
    let _user2 = api_client
        .get_user_info_by_account(GetUserInfoByAccountRequest {
            account_id: Some(user2_account_id.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .user
        .unwrap();

    let referral_reward = Tokenomics {
        stats: BlockchainStats::new(),
    }
    .get_referral_reward_amount()
    .await
    .unwrap();

    assert_eq!(
        user1_balance_pre + referral_reward - payment_amount,
        user1.balance
    );

    finalize_test().await;
}

/// Helper
async fn init_test() {
    enable_logger();
}

/// Helper
async fn finalize_test() {
    spawn(async {
        // stop the db so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        info!("resources cleanup completed");
    })
    .await
    .unwrap();
}
