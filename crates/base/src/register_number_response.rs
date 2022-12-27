// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::karma_coin::karma_coin_verifier::{RegisterNumberResponse, RegisterNumberResult};
use anyhow::Result;
use ed25519_dalek::{Keypair, Signer};

impl RegisterNumberResponse {
    // we can't implement default here due to prost::message required derivation
    pub fn new() -> Self {
        RegisterNumberResponse {
            result: 0,
            code: 0,
            signature: None,
        }
    }
}

impl From<RegisterNumberResult> for RegisterNumberResponse {
    fn from(result: RegisterNumberResult) -> Self {
        let mut resp = RegisterNumberResponse::new();
        resp.result = result as i32;
        resp
    }
}

impl RegisterNumberResponse {
    pub fn sign(&mut self, key_pair: &Keypair) -> Result<()> {
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
