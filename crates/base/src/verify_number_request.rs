// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use ed25519_dalek::{Keypair, Signer};
use crate::karma_coin::karma_coin_verifier::VerifyNumberRequest;

impl VerifyNumberRequest {

    // we can't implement default here due to prost::message required derivation
    pub fn new() -> Self {
        VerifyNumberRequest {
            account_id: None,
            mobile_number: None,
            code: 0,
            nickname: "".into(),
            signature: None,
        }
    }
}

impl VerifyNumberRequest {
    pub fn sign(
        &mut self,
        key_pair: &Keypair,
    ) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::karma_coin::karma_coin_core_types::Signature;
        self.signature = Some(Signature {
            scheme: 0,
            signature: key_pair.sign(&buf).as_ref().to_vec(),
        });

        Ok(())
    }
}