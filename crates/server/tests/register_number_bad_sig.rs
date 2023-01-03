#[path = "common/mod.rs"]
mod common;
use common::init_test;

use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair, MobileNumber};
use base::karma_coin::karma_coin_verifier::verifier_service_client::VerifierServiceClient;
use base::karma_coin::karma_coin_verifier::RegisterNumberRequest;
use server::server_service::{ServerService, Startup};
use xactor::Service;

/// Test invalid client signature
#[tokio::test(flavor = "multi_thread")]
async fn register_number_bad_signature_test() {
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

    let mut verifier_service = VerifierServiceClient::connect("http://[::1]:9888")
        .await
        .unwrap();

    let resp = verifier_service
        .register_number(register_number_request)
        .await;
    assert!(resp.is_err());
}
