// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::CharTrait;

impl CharTrait {
    pub fn new(id: u32, name: &str) -> Self {
        CharTrait {
            id,
            name: name.to_string(),
        }
    }
}
