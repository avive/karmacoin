// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::anyhow;
use base::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use base::karma_coin::karma_coin_api::api_service_client::ApiServiceClient;
use base::karma_coin::karma_coin_api::{SubmitTransactionRequest, SubmitTransactionResult};
use base::karma_coin::karma_coin_core_types::TransactionType::NewUserV1;
use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair, MobileNumber};
use base::karma_coin::karma_coin_core_types::{
    NewUserTransactionV1, SignedTransaction, TransactionData,
};
use base::karma_coin::karma_coin_verifier::verifier_service_client::VerifierServiceClient;
use base::karma_coin::karma_coin_verifier::VerifyNumberRequest;
use base::signed_trait::SignedTrait;
use base::tests_helpers::enable_logger;
use chrono::Utc;
use db::db_service::DatabaseService;
use log::info;
use prost::Message;
use xactor::*;

// helper function to create a new user
#[allow(dead_code)]
pub async fn create_user(user_name: String, number: String) -> Result<(KeyPair, MobileNumber)> {
    let user_key_pair = KeyPair::new();
    let user_ed_key_pair = user_key_pair.to_ed2559_keypair();
    let account_id_bytes = user_ed_key_pair.public.to_bytes().to_vec();

    let mobile_number = MobileNumber { number };
    let account_id = AccountId {
        data: account_id_bytes.clone(),
    };

    let mut verifier_service_client = VerifierServiceClient::connect("http://127.0.0.1:8080")
        .await
        .unwrap();

    let mut v_request = VerifyNumberRequest::new();

    v_request.mobile_number = Some(mobile_number.clone());
    v_request.account_id = Some(account_id.clone());
    v_request.requested_user_name = user_name.clone();
    v_request.signature = Some(v_request.sign(&user_ed_key_pair).unwrap());

    v_request
        .verify_signature()
        .expect("signature verification failed");

    let resp1 = verifier_service_client
        .verify_number(v_request)
        .await
        .unwrap();

    let v_resp = resp1.into_inner();

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
        timestamp: Utc::now().timestamp_millis() as u64,
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

    let mut api_client = ApiServiceClient::connect("http://[::1]:9080")
        .await
        .unwrap();

    let resp = api_client
        .submit_transaction(SubmitTransactionRequest {
            transaction: Some(signed_tx.clone()),
        })
        .await?
        .into_inner();

    if resp.submit_transaction_result != SubmitTransactionResult::Submitted as i32 {
        return Err(anyhow!("Transaction rejected"));
    }

    Ok((user_key_pair, mobile_number))
}

/// Helper
#[allow(dead_code)]
pub async fn init_test() {
    enable_logger();
}

/// Helper
#[allow(dead_code)]
pub async fn finalize_test() {
    spawn(async {
        // stop the db so it has a chance to destroy itself if it is configured to destroy storage on stop...
        let mut db_service = DatabaseService::from_registry().await.unwrap();
        let _ = db_service.stop(None);
        info!("resources cleanup completed");
    })
    .await
    .unwrap();
}
