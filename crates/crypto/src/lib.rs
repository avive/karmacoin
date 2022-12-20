// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

#[cfg_attr(test, macro_use)]
extern crate log;

extern crate base;
extern crate bytes;
extern crate curve25519_dalek;
extern crate ed25519_dalek;
extern crate hex;
extern crate rand;
extern crate sha2;
extern crate x25519_dalek;
/// Crypto crate should only contain state-less functions which are not concurrency-safe.
/// Clients of this create add concurrency safety and caching of results where applicable.
/// This design makes testing and security audits of this crate much simpler.
pub mod aead_cypher;
pub mod aes_cypher;
pub mod hasher;
pub mod hmacer;
pub mod kdfer;
pub mod utils;
pub mod x2dh;
