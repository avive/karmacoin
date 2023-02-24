// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

/// The Verifier module provides the KarmaCoin mobile phone verification api to users.
/// Users use the verifier to verify their mobile phone number on-chain.
/// For further details read the KarmaCoin docs and onboarding flows.
pub(crate) mod sms_invites_sender;
pub(crate) mod verifier_service;
pub(crate) mod verify_number;
