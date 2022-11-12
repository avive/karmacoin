// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{CharTrait, TraitName};

impl TraitName {
    pub fn new(char_trait: CharTrait, name: &str) -> Self {
        TraitName {
            char_trait: char_trait as i32,
            name: name.to_string()
        }
    }
}
