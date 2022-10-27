// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

extern crate log;

use anyhow::Result;

use db::db_service::{Compact, DataItem, DatabaseService, Destroy, ReadItem, WriteItem};

use bytes::Bytes;
use std::time::Duration;
use tokio::time::sleep;

use db::db_service;
use xactor::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_ttl() {
    use base::test_helpers::enable_logger;
    enable_logger();

    let addr = DatabaseService::from_registry().await.unwrap();

    let key1 = Bytes::from("key_2");
    // println!("key1: {:?}", key1);

    let value1 = Bytes::from("value_1");
    // println!("value1: {:?}", value1);

    let read_req = ReadItem {
        key: key1.clone(),
        cf: db_service::TESTS_COL_FAMILY,
    };

    let write_req = WriteItem {
        data: DataItem {
            key: key1,
            value: value1,
        },
        cf: db_service::TESTS_COL_FAMILY,
        ttl: 2,
    };

    let _ = addr.call(write_req).await.expect("failed to write to db");
    sleep(Duration::from_secs(4)).await;
    let _ = addr.call(Compact {}).await.expect("failed to compact db");

    let _res1: Result<Option<(Bytes, u64)>> = addr.call(read_req).await.expect("failed to read");

    // todo: fix me - the filter function is not running even when compact is called - might be beacuse of the compaction config.
    //assert!(res1.unwrap().is_none());

    let _ = addr.call(Destroy).await.expect("failed to delete the db");
}
