// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[path = "common/mod.rs"]
mod common;

use common::{create_user, finalize_test, init_test};

/// tests in this file should be run sequentially and not in parallel
use server::server_service::{ServerService, Startup};
use xactor::Service;

// Attempt to create new user with an existing username
#[tokio::test(flavor = "multi_thread")]
async fn new_user_existing_user_name() {
    init_test().await;

    // Start the server
    let server = ServerService::from_registry().await.unwrap();
    server.call(Startup {}).await.unwrap().unwrap();

    // todo: figure out why grpc warmup is needed - without the delay we have random connection refused
    // from api client
    use tokio::time::{sleep, Duration};
    sleep(Duration::from_millis(300)).await;

    create_user("avive".into(), "+972539805381".into())
        .await
        .unwrap();

    create_user("avive".into(), "+972549805382".into())
        .await
        .expect_err("should fail");

    finalize_test().await;
}
