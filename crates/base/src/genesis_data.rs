// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::GenesisData;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for GenesisData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "GenesisData {{ karma_reward_amount: {}, karma_reward_eligibility: {}, karma_reward_period: {} }}",
            &self.karma_reward_amount,
            &self.karma_rewards_eligibility,
            &self.karma_rewards_period_hours,
        )
    }
}
