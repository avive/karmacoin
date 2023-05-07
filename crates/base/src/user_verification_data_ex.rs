// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{UserVerificationDataEx, VerificationResult};

impl From<VerificationResult> for UserVerificationDataEx {
    fn from(value: VerificationResult) -> Self {
        UserVerificationDataEx {
            account_id: None,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            verification_result: value as i32,
            verifier_account_id: None,
            mobile_number: None,
            requested_user_name: "".into(),
        }
    }
}
