// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY,
};
use anyhow::{anyhow, Result};
use base::genesis_config_service::{AMBASSADOR_CHAR_TRAIT_ID, SPENDER_CHAR_TRAIT_ID};
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::{
    ExecutionResult, FeeType, PaymentTransactionV1, SignedTransaction, TransactionEvent,
    TransactionType, User,
};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use std::collections::HashMap;

/// Get the on-chain User for the tx payee.
/// Returns None if no user exists for the payee's mobile number
pub(crate) async fn get_payee_user(signed_transaction: &SignedTransaction) -> Result<Option<User>> {
    let tx_body = signed_transaction.get_body()?;

    let payment_tx: PaymentTransactionV1 = tx_body.get_payment_transaction_v1()?;
    let mobile_number: String = payment_tx.to.unwrap().number;

    // locate payee's account Id by mobile number
    let payee_account_id_data = DatabaseService::read(ReadItem {
        key: Bytes::from(mobile_number.as_bytes().to_vec()),
        cf: MOBILE_NUMBERS_COL_FAMILY,
    })
    .await?;

    if payee_account_id_data.is_none() {
        return Ok(None);
    }

    let payee_account_id = payee_account_id_data.unwrap().0.as_ref().to_vec();
    let payee_user_data = DatabaseService::read(ReadItem {
        key: Bytes::from(payee_account_id.clone()),
        cf: USERS_COL_FAMILY,
    })
    .await?;

    if payee_user_data.is_none() {
        return Ok(None);
    }

    Ok(Some(User::decode(payee_user_data.unwrap().0.as_ref())?))
}

impl BlockChainService {
    /// Process a payment transaction from payer to payee - update ledger state, emit tx event
    /// This is a helper method for the block creator and is used as part of block creation flow
    pub(crate) async fn process_payment_transaction(
        &mut self,
        signed_transaction: &SignedTransaction,
        payer: &mut User,
        payee: &mut User,
        sign_ups: &mut HashMap<Vec<u8>, SignedTransaction>,
        tokenomics: &Tokenomics,
        event: &mut TransactionEvent,
    ) -> Result<()> {
        let tx_hash = signed_transaction.get_hash()?;

        info!(
            "Processing payment transaction with hash {}",
            short_hex_string(tx_hash.as_ref())
        );

        // reject a payment from user to itself
        if payer.account_id.as_ref().unwrap().data == payee.account_id.as_ref().unwrap().data {
            return Err(anyhow!("You can't send karma coins to yourself"));
        }

        // validate the transaction
        signed_transaction.validate().await?;
        let tx_body = signed_transaction.get_body()?;

        // validate tx body and user nonce
        tx_body.validate(payer.nonce).await?;

        info!("Processing payment transaction: {}", signed_transaction);
        info!("Body: {}", tx_body);
        info!("From user: {}", payer);
        info!("To user: {}", payee);

        let payment_tx: PaymentTransactionV1 = tx_body.get_payment_transaction_v1()?;
        payment_tx.verify_syntax()?;

        if payer.account_id.as_ref().unwrap().data != payment_tx.from.as_ref().unwrap().data {
            return Err(anyhow!(
                "From account in payment tx must be the same as the signer account "
            ));
        }

        info!("Payment data: {}", payment_tx);

        let payee_mobile_number = payment_tx.to.unwrap().number;
        let payment_amount = payment_tx.amount;

        let apply_subsidy = tokenomics
            .should_subsidise_transaction_fee(0, tx_body.fee, TransactionType::PaymentV1)
            .await?;

        // actual fee amount to be paid by the user. 0 if fee is subsidised by the protocol
        let user_tx_fee_amount = if apply_subsidy { 0 } else { tx_body.fee };

        let fee_type = if apply_subsidy {
            FeeType::Mint
        } else {
            FeeType::User
        };

        if payer.balance < payment_amount + user_tx_fee_amount {
            // we reject the transaction and don't mint tx fee subsidy in this case
            // to avoid spamming the network with txs with insufficient funds
            return Err(anyhow!("payer has insufficient balance to pay"));
        }

        // update payee balance to reflect payment and tx fee (when applicable)
        payee.balance += payment_amount;

        // update payer balance to reflect payment
        payer.balance -= payment_amount + user_tx_fee_amount;

        if payment_tx.char_trait_id != 0 {
            // payment includes an appreciation for a character trait - update user character trait points
            payee.inc_trait_score(payment_tx.char_trait_id);
            event.appreciation_char_trait_idx = payment_tx.char_trait_id;
        }

        let referral_reward_amount = tokenomics.get_referral_reward_amount().await?;

        // apply new user referral reward to the payer if applicable
        if sign_ups.contains_key(payee_mobile_number.as_bytes()) {
            // remove from signups map to prevent double referral rewards for for the same new user
            sign_ups.remove(payee_mobile_number.as_bytes());

            // this is a new user referral payment tx - payer should get the referral fee!
            //let _sign_up_tx = sign_ups.get(mobile_number.as_bytes()).unwrap();
            // todo: award signer with the referral reward if applicable
            info!(
                "apply referral reward: {} to: {}",
                referral_reward_amount, payer.user_name
            );
            payer.balance += referral_reward_amount;

            // Give payer karma points for helping to grow the network
            payer.inc_trait_score(AMBASSADOR_CHAR_TRAIT_ID);
        };

        // Give payer karma points for spending karma coins
        payer.inc_trait_score(SPENDER_CHAR_TRAIT_ID);

        // index the transaction in the db by hash
        let mut tx_data = Vec::with_capacity(signed_transaction.encoded_len());
        info!(
            "binary transaction size: {}",
            signed_transaction.encoded_len()
        );

        signed_transaction.encode(&mut tx_data)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: tx_hash.clone(),
                value: Bytes::from(tx_data),
            },
            cf: TRANSACTIONS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // index the transaction in the db for both payer and payee
        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(payer.account_id.as_ref().unwrap().data.to_vec()),
        )
        .await?;

        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(payee.account_id.as_ref().unwrap().data.to_vec()),
        )
        .await?;

        // Update payer on chain account
        let mut buf = Vec::with_capacity(payer.encoded_len());
        payer.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(payer.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // Update payee on chain account
        let mut buf = Vec::with_capacity(payee.encoded_len());
        payee.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(payee.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // update tx event
        event.referral_reward = referral_reward_amount;
        event.fee_type = fee_type as i32;
        event.fee = tx_body.fee;
        event.result = ExecutionResult::Executed as i32;

        Ok(())
    }
}
