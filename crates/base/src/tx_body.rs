// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::genesis_config_service::{GenesisConfigService, NET_ID_KEY};
use crate::karma_coin::karma_coin_core_types::{
    DeleteUserTransactionV1, NewUserTransactionV1, PaymentTransactionV1, TransactionBody,
    TransactionType, UpdateUserTransactionV1,
};
use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use log::info;
use prost::Message;
use std::fmt;
use std::fmt::{Display, Formatter};

impl Display for TransactionBody {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction {{ timestamp: {}, nonce : {}, fee: {}, netid: {} }}",
            self.timestamp, self.nonce, self.fee, self.net_id
        )
    }
}

impl TransactionBody {
    /// Validate transaction has valid syntax, fields has the correct net id and is properly
    /// signed before processing it
    pub async fn validate(&self, _user_nonce: u64) -> Result<()> {
        // skip nonce validation for now until the design is fully sorted out
        //self.validate_nonce(user_nonce)?;
        self.verify_syntax().await?;
        self.verify_timestamp()?;
        self.verify_tx_fee()
    }

    /// Validate tx nonce against user's current nonce
    pub fn validate_nonce(&self, user_nonce: u64) -> Result<()> {
        info!(
            "validating tx nonce. tx nonce: {}, user_nonce: {}",
            self.nonce, user_nonce
        );
        if self.nonce <= user_nonce {
            return Err(anyhow!(
                "invalid nonce in tx. Got: {}, Expected at least: {}",
                self.nonce,
                user_nonce + 1
            ));
        }
        Ok(())
    }

    pub fn verify_tx_fee(&self) -> Result<()> {
        if self.fee == 0 {
            return Err(anyhow!("tx fee must be positive"));
        }
        Ok(())
    }

    pub async fn verify_syntax(&self) -> Result<()> {
        if self.transaction_data.is_none() {
            return Err(anyhow!("required transaction data is missing"));
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

    pub fn get_delete_user_transaction_v1(&self) -> Result<DeleteUserTransactionV1> {
        let data = self
            .transaction_data
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx data"))?;
        if data.transaction_type != TransactionType::DeleteUserV1 as i32 {
            return Err(anyhow!("unexpected transaction type"));
        }

        Ok(DeleteUserTransactionV1::decode(
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
