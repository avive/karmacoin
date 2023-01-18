// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY,
};
use base::karma_coin::karma_coin_core_types::{
    ExecutionResult, FeeType, SignedTransaction, TransactionEvent, User,
};
use base::signed_trait::SignedTrait;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;

pub(crate) struct NewUserProcessingResult {
    pub(crate) mobile_number: String,
}

impl BlockChainService {
    /// Process a new user transaction - update ledger state, emit tx event
    /// This method will not add the tx to a block and is used as part of block creation flow
    pub(crate) async fn process_new_user_transaction(
        &mut self,
        transaction: &SignedTransaction,
        tokenomics: &Tokenomics,
        event: &mut TransactionEvent,
    ) -> Result<NewUserProcessingResult> {
        let account_id = transaction
            .signer
            .as_ref()
            .ok_or_else(|| anyhow!("missing account id in tx"))?;

        let tx_hash = transaction.get_hash()?;

        // validate tx syntax, fields, signature, net_id before processing it
        transaction.validate(0).await?;

        let tx_fee = transaction.fee;
        let new_user_tx = transaction.get_new_user_transaction_v1()?;

        let verification_evidence = new_user_tx
            .verify_number_response
            .ok_or_else(|| anyhow!("missing verifier data"))?;

        // verify evidence signature
        // todo: verify verifier is valid according to consensus rules
        // and genesis config
        verification_evidence.verify_signature()?;

        let mobile_number = verification_evidence
            .mobile_number
            .ok_or_else(|| anyhow!("missing mobile number in verifier data"))?;

        let evidence_account_id = verification_evidence
            .account_id
            .ok_or_else(|| anyhow!("missing account id in verifier data"))?;

        if account_id.data != evidence_account_id.data {
            return Err(anyhow!("account id mismatch"));
        }

        let mut user = User {
            account_id: Some(account_id.clone()),
            nonce: 1,
            user_name: verification_evidence.user_name.clone(),
            mobile_number: Some(mobile_number.clone()),
            balance: 0,
            trait_scores: vec![],
            pre_keys: vec![],
        };

        let apply_subsidy = tokenomics
            .should_subsidise_transaction_fee(0, tx_fee)
            .await?;

        let signup_reward_amount = tokenomics.get_signup_reward_amount().await?;
        let user_tx_fee = if apply_subsidy { 0 } else { tx_fee };

        if !apply_subsidy && tx_fee >= signup_reward_amount {
            // invalid tx - tx fee is higher than the block award
            return Err(anyhow!(
                "tx fee is greater than signup reward and no tx fee subsidy is applied"
            ));
        }

        let fee_type = if apply_subsidy {
            FeeType::Mint
        } else {
            FeeType::User
        };

        // Check user account id is not already on chain
        if (DatabaseService::read(ReadItem {
            key: Bytes::from(user.account_id.as_ref().unwrap().data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?)
            .is_some()
        {
            return Err(anyhow!("User with provided account id already exists on chain. You can use an update tx to update it"));
        }

        user.balance += signup_reward_amount - user_tx_fee;

        // todo: figure out personality trait for joiner - brave? ahead of the curve?
        user.trait_scores = vec![];

        // add the new user to db

        // todo: update existing user if it exists - this will happen for a block producer or a verifier

        let mut buf = Vec::with_capacity(user.encoded_len());
        user.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account_id.data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // update nickname index
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(verification_evidence.user_name.as_bytes().to_vec()),
                value: Bytes::from(account_id.data.to_vec()),
            },
            cf: USERS_NAMES_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // update mobile numbers index
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(mobile_number.number.as_bytes().to_vec()),
                value: Bytes::from(account_id.data.to_vec()),
            },
            cf: MOBILE_NUMBERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        let mut tx_data = Vec::with_capacity(transaction.encoded_len());
        info!("binary transaction size: {}", transaction.encoded_len());

        transaction.encode(&mut tx_data)?;

        // index the transaction in the db by hash
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: tx_hash.clone(),
                value: Bytes::from(tx_data),
            },
            cf: TRANSACTIONS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // index the transaction in the db by signer account id
        self.index_transaction_by_account_id(transaction, Bytes::from(account_id.data.to_vec()))
            .await?;

        event.fee_type = fee_type as i32;
        event.fee = tx_fee;
        event.signup_reward = signup_reward_amount;
        event.result = ExecutionResult::Executed as i32;

        Ok(NewUserProcessingResult {
            mobile_number: mobile_number.number.clone(),
        })
    }
}
