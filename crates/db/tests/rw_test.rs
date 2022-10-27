// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[cfg_attr(test, macro_use)]
extern crate log;

use db::db_service::{DataItem, DatabaseService, Destroy, ReadItem, WriteItem};

use base::test_helpers::enable_logger;
use bytes::Bytes;
use db::db_service;
use xactor::*;

#[tokio::test]
async fn test_read_write() {
    enable_logger();

    let addr = DatabaseService::from_registry().await.unwrap();

    let key1 = Bytes::from("key_1");
    debug!("key1: {:?}", key1);

    let value1 = Bytes::from("value_1");
    debug!("value1: {:?}", value1);

    let value2 = value1.clone();

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
        ttl: 0,
    };

    addr.call(write_req)
        .await
        .expect("failed to write to db")
        .expect("");

    let res: Result<Option<(Bytes, u64)>> = addr.call(read_req).await.expect("failed to read");

    let data = res.expect("expected data from db").unwrap();
    assert_eq!(data.0, value2, "expected to get stored value");

    let _ = addr.call(Destroy).await.expect("failed to delete the db");
}

#[tokio::test]
async fn test_read_write_string_keys() {
    enable_logger();

    let addr = DatabaseService::from_registry().await.unwrap();

    let value1 = Bytes::from("value_1");
    debug!("value1: {:?}", value1);

    let value2 = value1.clone();

    let read_req = ReadItem {
        key: bytes::Bytes::from("key_1".as_bytes()),
        cf: db_service::TESTS_COL_FAMILY,
    };

    let write_req = WriteItem {
        data: DataItem {
            key: bytes::Bytes::from("key_1".as_bytes()),
            value: value1,
        },
        cf: db_service::TESTS_COL_FAMILY,
        ttl: 0,
    };

    let _ = addr
        .call(write_req)
        .await
        .expect("failed to write to db")
        .expect("");

    let res: Result<Option<(Bytes, u64)>> = addr.call(read_req).await.expect("failed to read");

    let data = res.expect("expected data from db").unwrap();
    assert_eq!(data.0, value2, "expected to get stored value");

    let _ = addr.call(Destroy).await.expect("failed to delete the db");
}
