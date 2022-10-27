// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

extern crate rocksdb;

use anyhow::{bail, Result};

// use self::rocksdb::IteratorMode;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use datetime::Instant;
use rocksdb::CompactionDecision;

// Low-level database util functions

// Private helper function - returns compaction decision based on ttl
pub(crate) fn filter_data(_level: u32, key: &[u8], value: &[u8]) -> CompactionDecision {
    use self::CompactionDecision::*;

    debug!("Filtering data function...");

    if let Ok((_, ttl)) = parse_value(value) {
        let now_seconds = Instant::now().seconds() as u64;
        debug!("Now timestamp {:?} seconds", now_seconds);

        if ttl == 0 {
            debug!("item with 0 ttl should not be removed");
            Keep
        } else if now_seconds > ttl {
            debug!("data expired for key: {:?} - removing", key);
            Remove
        } else {
            debug!("data not expired yet fork key: {:?} - keeping", key);
            Keep
        }
    } else {
        debug!("invalid data - removing");
        Remove
    }
}

// Private helper - break binary value into its ttl and data parts
pub(crate) fn parse_value(data: &[u8]) -> Result<(Bytes, u64)> {
    if data.len() < 8 {
        bail!("unexpected small value");
    }

    let bytes = Bytes::copy_from_slice(data);

    debug!("Total raw value len: {:?}", bytes.len());
    let ttl_bytes = bytes.slice(0..8);

    // ttl is persisted as absolute seconds from unix EPOCH but we need to return them as
    // in seconds units based on the current time
    let time_stamp = BigEndian::read_u64(ttl_bytes.as_ref());
    debug!("Parse data: time_stamp is {:?} seconds", time_stamp);

    let curr_ttl = time_stamp as i64 - Instant::now().seconds();
    debug!("Parse data: current ttl is {:?}", curr_ttl);

    if curr_ttl < 0 {
        debug!("Data item with negative ttl");
    }

    let ttl = std::cmp::max(0, curr_ttl) as u64;
    let data_bytes = bytes.slice(8..);

    debug!("Value len: {:?}", data_bytes.len());
    debug!("Computed ttl: {:?}", ttl);

    Ok((data_bytes, ttl))
}
