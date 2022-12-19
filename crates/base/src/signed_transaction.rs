// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use ed25519_dalek::Verifier;
use prost::Message;
use crate::karma_coin::karma_coin_core_types::{NewUserTransactionV1, PaymentTransactionV1, SignedTransaction, TransactionType, UpdateUserV1};

impl SignedTransaction {

    /// Validate data of a newly submitted transaction before processing it
    pub fn validate(&self, user_nonce: u64) -> Result<()> {
        if self.nonce != user_nonce + 1 {
            return Err(anyhow!("expected nonce to be user's nonce plus 1"));
        }

        // check timestamp is close to now - within 48 hours
        let now = Utc::now().timestamp_nanos() as u64;
        if i64::abs(now as i64 - self.timestamp as i64) > Duration::hours(48).num_nanoseconds().unwrap() {
           return Err(anyhow!("invalid timestamp - too far from now"));
        }

        self.verify_signature()
    }

    /// Verify the signer's signature
    pub fn verify_signature(&self) -> Result<()> {

        let mut cloned = self.clone();
        cloned.signature = None;

        let mut buf = Vec::with_capacity(cloned.encoded_len());
        if cloned.encode(&mut buf).is_err() {
            return Err(anyhow!("invalid binary tx data"));
        };

        let signer = self.signer.as_ref().ok_or_else(|| anyhow!("missing account id"))?;
        let signature_data = self.signature.as_ref().ok_or_else(|| anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(signer.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature).map_err(|_| anyhow!("invalid signature"))
    }

    /// Returns the transaction type
    pub fn get_tx_type(&self) -> Result<TransactionType> {
        let data = self.transaction_data.as_ref().ok_or_else(|| anyhow!("missing tx data"))?;
        TransactionType::from_i32(data.transaction_type).ok_or_else(|| anyhow!("unexpected tx type"))
    }

    pub fn get_new_user_transaction_v1(&self) -> Result<NewUserTransactionV1> {
        let data = self.transaction_data.as_ref().ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::NewUserV1 as i32 {
            return Err(anyhow!("unexpected transaction type"))
        }

        Ok(NewUserTransactionV1::decode(data.transaction_data.as_ref())?)
    }

    pub fn get_payment_transaction_v1(&self) -> Result<PaymentTransactionV1> {
        let data = self.transaction_data.as_ref().ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::PaymentV1 as i32 {
            return Err(anyhow!("unexpected transaction type"))
        }

        Ok(PaymentTransactionV1::decode(data.transaction_data.as_ref())?)
    }

    pub fn get_update_user_transaction_v1(&self) -> Result<UpdateUserV1> {
        let data = self.transaction_data.as_ref().ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::UpdateUserV1 as i32 {
            return Err(anyhow!("unexpected transaction type"))
        }

        Ok(UpdateUserV1::decode(data.transaction_data.as_ref())?)
    }
}