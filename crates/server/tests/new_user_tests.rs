// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

use base::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{SubmitTransactionRequest, SubmitTransactionResult};
use base::karma_coin::karma_coin_core_types::TransactionType::UpdateUserV1;
use base::karma_coin::karma_coin_core_types::VerifyNumberResult::Verified;
use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair, MobileNumber, User};
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
use xactor::*;

/// tests in this file should be run sequentially and not in parallel

/// Test complete user signup flow
#[tokio::test(flavor = "multi_thread")]
async fn new_user_happy_flow_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    let client_key_pair = KeyPair::new();
    let client_ed_key_pair = client_key_pair.to_ed2559_keypair();
    let account_id_bytes = client_ed_key_pair.public.to_bytes().to_vec();

    let mobile_number = MobileNumber {
        number: "972549805380".to_string(),
    };
    let account_id = AccountId {
        data: account_id_bytes.clone(),
    };
    let user_name = "avive";

    let mut register_number_request = RegisterNumberRequest::new();
    register_number_request.mobile_number = Some(mobile_number.clone());
    register_number_request.account_id = Some(account_id.clone());

    register_number_request.signature =
        Some(register_number_request.sign(&client_ed_key_pair).unwrap());

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

    // in production this code is obtained from sms message
    v_request.code = code;

    // user's requested nickname
    v_request.nickname = user_name.into();
    v_request.signature = Some(v_request.sign(&client_ed_key_pair).unwrap());

    let resp1 = verifier_service_client
        .verify_number(v_request)
        .await
        .unwrap();

    let v_resp = resp1.into_inner();
    assert_eq!(v_resp.result, Verified as i32);

    let user = User::new(account_id.clone(), user_name.into(), mobile_number);

    let new_user_tx = NewUserTransactionV1::new(user, v_resp.clone());

    let mut buf = Vec::with_capacity(new_user_tx.encoded_len());
    new_user_tx.encode(&mut buf).unwrap();

    let network_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
        .await
        .unwrap()
        .unwrap() as u32;

    let mut signed_tx = SignedTransaction {
        signer: Some(account_id.clone()),
        timestamp: Utc::now().timestamp_millis() as u64,
        nonce: 0,
        fee: 10,
        transaction_data: Some(TransactionData {
            transaction_data: buf,
            transaction_type: UpdateUserV1 as i32,
        }),
        network_id,
        signature: None,
    };

    signed_tx.signature = Some(signed_tx.sign(&client_ed_key_pair).unwrap());

    // drop the db
    finalize_test().await;

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
