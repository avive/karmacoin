// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;

// use base::server_config_service::{GetValue, ServerConfigService, SetValue};
use base::test_helpers::enable_logger;
use db::db_service::DatabaseService;
// use server::server_service::ServerService;

use xactor::*;

async fn init_test() {
    // init db
    // DbConfigService::from_registry().await.unwrap();
}

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

#[tokio::test(flavor = "multi_thread")]
async fn register_number() {
    init_test().await;
    enable_logger();
    finalize_test().await;
}
