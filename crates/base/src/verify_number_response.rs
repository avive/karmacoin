use crate::karma_coin::karma_coin_core_types::{VerifyNumberResponse, VerifyNumberResult};
use anyhow::Result;
use ed25519_dalek::{Keypair, Signer};
use chrono::prelude::*;

impl VerifyNumberResponse {

    // we can't implement default here due to prost::message required derivation
    fn new() -> Self {
        VerifyNumberResponse {
            timestamp: Utc::now().timestamp_nanos() as u64,
            result: 0,
            nickname : "".into(),
            account_id: None,
            mobile_number: None,
            signature: None
        }
    }
}

impl From<VerifyNumberResult> for VerifyNumberResponse {
    fn from(result: VerifyNumberResult) -> Self {
        let mut resp = VerifyNumberResponse::new();
        resp.result = result as i32;
        resp
    }
}

impl VerifyNumberResponse {

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