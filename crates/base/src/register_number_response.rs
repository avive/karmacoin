use anyhow::Result;
use ed25519_dalek::{Keypair, Signer};
use chrono::prelude::*;
use crate::karma_coin::karma_coin_verifier::{RegisterNumberResponse, RegisterNumberResult};

impl RegisterNumberResponse {

    // we can't implement default here due to prost::message required derivation
    fn new() -> Self {
        RegisterNumberResponse {
            result: 0,
            signature: None
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

    pub fn sign(
        &mut self,
        key_pair: &Keypair,
    ) -> Result<()> {
        use prost::Message;
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        use crate::karma_coin::karma_coin_core_types::Signature;
        self.signature = Some(Signature {
            scheme_id: 0,
            signature: key_pair.sign(&buf).as_ref().to_vec(),
        });

        let mut buf1 = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf1)?;

        self.signature = Some(Signature {
            scheme_id: 0,
            signature: key_pair.sign(&buf1).as_ref().to_vec(),
        });
        Ok(())
    }
}