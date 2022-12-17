// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

/// Blockchain module provides low-level blockchain functionality such as creating blocks and processing transactions

pub (crate) mod blockchain_service;
pub (crate) mod get_head_height;
pub (crate) mod create_block;
pub (crate) mod new_user_tx_processor_v1;
pub (crate) mod payment_tx_processor_v1;
pub (crate) mod update_tx_processor_v1;