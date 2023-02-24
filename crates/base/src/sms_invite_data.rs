// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_core_types::{AccountId, MobileNumber};
use crate::karma_coin::karma_coin_verifier::SmsInviteMetadata;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for SmsInviteMetadata {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "signed_tx.rs {{ todo: implement me }}",)
    }
}

impl SmsInviteMetadata {
    pub fn new(
        mobile_number: &MobileNumber,
        inviter_account: &AccountId,
        invite_tx_hash: &[u8],
    ) -> Self {
        SmsInviteMetadata {
            mobile_number: Some(mobile_number.clone()),
            last_message_sent_time_stamp: 0,
            messages_sent: 0,
            inviter_account_id: Some(inviter_account.clone()),
            invite_tx_hash: invite_tx_hash.to_vec(),
        }
    }
}
