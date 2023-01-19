// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::{
    NewUserTransactionV1, PaymentTransactionV1, SignedTransaction, TransactionType,
    UpdateUserTransactionV1,
};
use crate::signed_trait::SignedTrait;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use chrono::{Duration, Utc};
use ed25519_dalek::{PublicKey, Signature};
use orion::hazardous::hash::sha2::sha256::Sha256;
use prost::Message;
use std::fmt;
use std::fmt::{Display, Formatter};

impl SignedTrait for SignedTransaction {
    fn get_sign_message(&self) -> Result<Vec<u8>> {
        let mut cloned = self.clone();
        cloned.signature = None;
        let mut buf = Vec::with_capacity(cloned.encoded_len());
        cloned.encode(&mut buf)?;
        Ok(buf.to_vec())
    }

    fn get_signature(&self) -> Result<Signature> {
        Ok(Signature::from_bytes(
            &self
                .signature
                .as_ref()
                .ok_or_else(|| anyhow!("no signature found"))?
                .signature
                .clone(),
        )?)
    }

    fn get_public_key(&self) -> Result<PublicKey> {
        Ok(PublicKey::from_bytes(
            self.signer
                .as_ref()
                .ok_or_else(|| anyhow!("missing key data"))?
                .data
                .as_slice(),
        )?)
    }
}

impl Display for SignedTransaction {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "SignedTransaction {{ from: {}, hash: {}, nonce: {}, fee: {}, time: {}, \
            net_id: {} }}",
            short_hex_string(&self.signer.as_ref().unwrap().data),
            short_hex_string(self.get_hash().unwrap().as_ref()),
            self.nonce,
            self.fee,
            self.timestamp,
            self.net_id
        )
    }
}

impl SignedTransaction {
    /// Returns the transaction canonical hash
    ///
    pub fn get_hash(&self) -> Result<Bytes> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf)?;

        // todo: refactor it to a canonical hash function for the project
        let mut hasher = Sha256::new();
        hasher.update(buf.as_ref())?;
        let digest = hasher.finalize()?;
        Ok(Bytes::from(digest.as_ref().to_vec()))

        //orion::hash::digest(&buf).map_err(|e| anyhow!("failed to hash data: {}", e))?;

        //Ok(Bytes::from(hash.as_ref().to_vec()));
        // unimplemented!("use rust crypto lib blake3 instead of orion");
    }

    /// Validate transaction has valid syntax, fields has the correct net id and is preorply
    /// signed before processing it
    pub async fn validate(&self, user_nonce: u64) -> Result<()> {
        if self.nonce != user_nonce + 1 {
            return Err(anyhow!(
                "Invalid nonce in tx. Got: {}, Expected: {}",
                self.nonce,
                user_nonce + 1
            ));
        }

        self.verify_syntax().await?;
        self.verify_timestamp()?;
        self.verify_tx_fee()?;
        self.verify_signature()
    }

    pub fn verify_tx_fee(&self) -> Result<()> {
        if self.fee == 0 {
            return Err(anyhow!("fee must be positive"));
        }
        Ok(())
    }

    pub async fn verify_syntax(&self) -> Result<()> {
        if self.signer.is_none() {
            return Err(anyhow!("signer is required"));
        }

        if self.signature.is_none() {
            return Err(anyhow!("signature is required"));
        }

        if self.transaction_data.is_none() {
            return Err(anyhow!("transaction data is required"));
        }

        let net_id = GenesisConfigService::get_u64(NET_ID_KEY.into())
            .await?
            .unwrap();

        if self.net_id as u64 != net_id {
            return Err(anyhow!(
                "Transaction has wrong net id - expected: {}, got: {}",
                net_id,
                self.net_id
            ));
        }

        Ok(())
    }

    /// Verify that tx is not too old
    pub fn verify_timestamp(&self) -> Result<()> {
        let now = Utc::now().timestamp_millis() as u64;

        if i64::abs(now as i64 - self.timestamp as i64) > Duration::hours(48).num_milliseconds() {
            return Err(anyhow!("invalid timestamp - too far from now"));
        }
        Ok(())
    }

    /// Returns the transaction type
    pub fn get_tx_type(&self) -> Result<TransactionType> {
        let data = self
            .transaction_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx data"))?;
        TransactionType::from_i32(data.transaction_type)
            .ok_or_else(|| anyhow!("unexpected tx type"))
    }

    pub fn get_new_user_transaction_v1(&self) -> Result<NewUserTransactionV1> {
        let data = self
            .transaction_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::NewUserV1 as i32 {
            return Err(anyhow!("unexpected transaction type"));
        }

        Ok(NewUserTransactionV1::decode(
            data.transaction_data.as_ref(),
        )?)
    }

    pub fn get_payment_transaction_v1(&self) -> Result<PaymentTransactionV1> {
        let data = self
            .transaction_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::PaymentV1 as i32 {
            return Err(anyhow!("unexpected transaction type"));
        }

        Ok(PaymentTransactionV1::decode(
            data.transaction_data.as_ref(),
        )?)
    }

    pub fn get_update_user_transaction_v1(&self) -> Result<UpdateUserTransactionV1> {
        let data = self
            .transaction_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::UpdateUserV1 as i32 {
            return Err(anyhow!("unexpected transaction type"));
        }

        Ok(UpdateUserTransactionV1::decode(
            data.transaction_data.as_ref(),
        )?)
    }
}
