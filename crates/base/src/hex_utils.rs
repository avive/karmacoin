// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate custom_error;
extern crate env_logger;
extern crate hex;

use anyhow::Result;
use hex::FromHexError;
use std::fmt;
use std::fmt::Formatter;
// hex formatting utils

/// helper function to format bytes as 0x1234..5678. Data must be at least 4 bytes long
pub fn hex_format(data: &[u8], f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", hex_string(data))
}

/// helper function to format bytes as 0x1234..5678. Data must be at least 4 bytes long
pub fn short_hex_format(data: &[u8], f: &mut Formatter<'_>) -> fmt::Result {
    let (prefix, body) = data.split_at(2);
    let (_, suffix) = body.split_at(body.len() - 2);

    let prefix = hex::encode(prefix);
    let suffix = hex::encode(suffix);

    write!(f, "0x{}..{}", prefix, suffix)
}

/// Returns the 0x prefixed hex string of input bytes
pub fn short_hex_string(data: &[u8]) -> String {
    let (prefix, body) = data.split_at(2);
    let (_, suffix) = body.split_at(body.len() - 2);

    let prefix = hex::encode(prefix);
    let suffix = hex::encode(suffix);

    format!("0x{}..{}", prefix, suffix)
}

/// Returns the 0x prefixed hex string of input bytes
pub fn hex_string(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

pub fn hex_from_string(data: String) -> Result<Vec<u8>, FromHexError> {
    hex::decode(data)
}
