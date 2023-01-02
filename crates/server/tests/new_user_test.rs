use crate::helpers::{create_user, finalize_test, init_test};
use server::server_service::{ServerService, Startup};
use xactor::Service;

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
