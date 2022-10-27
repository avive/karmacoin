// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use std::convert::From;

// An integer database key
pub struct IntDbKey(pub Bytes);

// Convert a u64 to a binary db key
impl From<u64> for IntDbKey {
    fn from(item: u64) -> Self {
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, item);
        IntDbKey(Bytes::from(buf.to_vec()))
    }
}
