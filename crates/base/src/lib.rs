// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate core;
extern crate serde;

pub mod block;
pub mod block_event;
pub mod blockchain_stats;
pub mod char_trait;
pub mod client_config_service;
pub mod genesis_config_service;
pub mod hasher;
pub mod hex_utils;
pub mod karma_coin;
pub mod key_pair;
pub mod logging_service;
pub mod new_user_tx;
pub mod payment_tx;
pub mod server_config_service;
pub mod signed_trait;
pub mod signed_tx;
pub mod sms_invite_data;
pub mod tests_helpers;
pub mod transaction_event;
pub mod tx_body;
pub mod update_user_tx;
pub mod user;
pub mod user_verification_data;
pub mod verify_number_request;

pub const GRPC_DESCRIPTOR: &[u8] = include_bytes!("karma_coin/descriptor.bin");
