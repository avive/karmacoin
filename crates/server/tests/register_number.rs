// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate log;

#[path = "common/mod.rs"]
mod common;
use common::{finalize_test, init_test};

use base::karma_coin::karma_coin_core_types::VerifyNumberResult::Verified;
use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair, MobileNumber};
use base::karma_coin::karma_coin_verifier::verifier_service_client::VerifierServiceClient;
use base::karma_coin::karma_coin_verifier::RegisterNumberResult::CodeSent;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, VerifyNumberRequest};
use base::signed_trait::SignedTrait;
use log::info;
use server::server_service::{ServerService, Startup};
use xactor::Service;

/// Test complete registration flow including sms verification
#[tokio::test(flavor = "multi_thread")]
async fn register_number_happy_flow_test() {
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
    v_request.mobile_number = Some(MobileNumber {
        number: "972549805380".to_string(),
    });
    v_request.account_id = Some(AccountId { data: account_id });

    // in production this code is obtained from sms message
    v_request.code = code;

    // user's requested nickname
    v_request.nickname = "avive".into();
    v_request.signature = Some(v_request.sign(&client_ed_key_pair).unwrap());

    let resp1 = verifier_service_client
        .verify_number(v_request)
        .await
        .unwrap();

    let v_resp = resp1.into_inner();
    assert_eq!(v_resp.result, Verified as i32);

    // drop the db
    finalize_test().await;
}
