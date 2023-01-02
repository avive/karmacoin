// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

use base::karma_coin::karma_coin_core_types::{
    AccountId, KeyPair, MobileNumber, VerifyNumberResult,
};
use base::karma_coin::karma_coin_verifier::verifier_service_client::VerifierServiceClient;
use base::karma_coin::karma_coin_verifier::RegisterNumberResult::CodeSent;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, VerifyNumberRequest};
use base::signed_trait::SignedTrait;
use base::test_helpers::enable_logger;
use db::db_service::DatabaseService;
use server::server_service::{ServerService, Startup};
use xactor::*;

mod register_number_tests;

/// Test attempt to register with a taken nickname
#[tokio::test(flavor = "multi_thread")]
async fn verifier_nickname_taken_test() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    let client_key_pair = KeyPair::new();
    let client_ed_key_pair = client_key_pair.to_ed2559_keypair();

    let mut register_number_request = RegisterNumberRequest::new();
    register_number_request.mobile_number = Some(MobileNumber {
        number: "972549805380".to_string(),
    });
    let account_id = client_ed_key_pair.public.to_bytes().to_vec();
    register_number_request.account_id = Some(AccountId {
        data: account_id.clone(),
    });
    register_number_request.signature =
        Some(register_number_request.sign(&client_ed_key_pair).unwrap());

    let mut verifier_service = VerifierServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    let resp = verifier_service
        .register_number(register_number_request)
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.result, CodeSent as i32);

    // obtain the verification code from the result as there's no sms service yet
    let code = resp.code;

    let mut v_request = VerifyNumberRequest::new();
    v_request.mobile_number = Some(MobileNumber {
        number: "972549805380".to_string(),
    });
    v_request.account_id = Some(AccountId { data: account_id });

    // in production this code is obtained from sms message
    v_request.code = code;

    // user's requested nickname
    v_request.nickname = "avive".into();
    v_request.signature = Some(v_request.sign(&client_ed_key_pair).unwrap());

    let resp1 = verifier_service.verify_number(v_request).await.unwrap();

    let v_resp = resp1.into_inner();
    assert_eq!(v_resp.result, VerifyNumberResult::Verified as i32);

    // now try to create a user with the same nickname

    let client1_key_pair = KeyPair::new();
    let client1_ed_key_pair = client1_key_pair.to_ed2559_keypair();
    let account1_id = client1_ed_key_pair.public.to_bytes().to_vec();
    let mut register_number_request1 = RegisterNumberRequest::new();
    register_number_request1.mobile_number = Some(MobileNumber {
        number: "972549805381".to_string(),
    });
    register_number_request1.account_id = Some(AccountId {
        data: account1_id.clone(),
    });
    register_number_request1.signature =
        Some(register_number_request1.sign(&client1_ed_key_pair).unwrap());

    let resp = verifier_service
        .register_number(register_number_request1)
        .await
        .unwrap()
        .into_inner();
    assert_eq!(resp.result, CodeSent as i32);
    // obtain the verification code from the result as there's no sms service yet
    let code1 = resp.code;

    let mut v_request1 = VerifyNumberRequest::new();
    v_request1.mobile_number = Some(MobileNumber {
        number: "972549805381".to_string(),
    });
    v_request1.account_id = Some(AccountId { data: account1_id });

    // in production this code is obtained from sms message
    v_request1.code = code1;

    // request nickname already reserved by another registration
    v_request1.nickname = "avive".into();
    v_request1.signature = Some(v_request1.sign(&client1_ed_key_pair).unwrap());
    let resp1 = verifier_service.verify_number(v_request1).await.unwrap();
    let v_resp = resp1.into_inner();
    assert_eq!(v_resp.result, VerifyNumberResult::NicknameTaken as i32);

    // drop the db
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
