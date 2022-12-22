// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::collections::HashMap;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_core_types::{CoinType, PaymentTransactionV1, SignedTransaction, User};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY};


/// Get the on-chain User for the tx payee.
/// Returns None if no user exists for the payee mobile number
pub(crate) async fn get_payee_user(tx: &SignedTransaction) -> Result<Option<User>> {
    let payment_tx: PaymentTransactionV1 = tx.get_payment_transaction_v1()?;
    let mobile_number: String = payment_tx.to.unwrap().number;

    // locate payee by mobile number
    let payee_account_id_data = DatabaseService::read(ReadItem {
        key: Bytes::from(mobile_number.as_bytes().to_vec()),
        cf: MOBILE_NUMBERS_COL_FAMILY
    }).await?;

    if payee_account_id_data.is_none() {
        return Ok(None)
    }

    let payee_account_id = payee_account_id_data.unwrap().0.as_ref().to_vec();
    let payee_user_data = DatabaseService::read(ReadItem {
        key: Bytes::from(payee_account_id.clone()),
        cf: USERS_COL_FAMILY
    }).await?;

    if payee_user_data.is_none() {
        return Ok(None)
    }

    Ok(Some(User::decode(payee_user_data.unwrap().0.as_ref())?))

}

/// Process a new user transaction - update ledger state, emit tx event
/// This method will not add the tx to a block nor index it
/// This is a helper method for the block creator
pub(crate) async fn process_transaction(
    transaction: &SignedTransaction,
    payer: &mut User,
    payee: &mut User,
    sign_ups: &HashMap<Vec<u8>, SignedTransaction>) -> Result<()> {

    transaction.validate(payer.nonce).await?;

    let payment_tx: PaymentTransactionV1 = transaction.get_payment_transaction_v1()?;
    payment_tx.verify_syntax()?;

    let mobile_number = payment_tx.to.unwrap().number;
    let payment = payment_tx.amount.unwrap();

    // check that payer has sufficient balance to pay
    let coin_type = CoinType::from_i32(payment.coin_type).ok_or_else(|| anyhow!("invalid coin type"))?;

    if payer.get_balance(coin_type).value < payment.value {
        return Err(anyhow!("payer has insufficient balance to pay"))
    }

    // update payee balance to reflect payment
    let mut balance = payee.get_balance(coin_type);
    balance.value += payment.value;
    payee.update_balance(&balance);

    // update payer balance to reflect payment
    let mut payer_balance = payer.get_balance(coin_type);
    payer_balance.value -= payment.value;
    payer.update_balance(&payer_balance);


    if sign_ups.contains_key(mobile_number.as_bytes()) {
        // this is a new user referral payment
        let _sign_up_tx = sign_ups.get(mobile_number.as_bytes()).unwrap();
        // todo: award signer with the referral reward if applicable
    };

    // index the transaction in the db by hash
    let mut tx_data = Vec::with_capacity(transaction.encoded_len());
    transaction.encode(&mut tx_data)?;
    let tx_hash = transaction.get_hash()?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: tx_hash.clone(),
            value: Bytes::from(tx_data)
        },
        cf: TRANSACTIONS_COL_FAMILY,
        ttl: 0,
    }).await?;

    // Update both payer and payee users accounts in the ledger

    // Update payer
    let mut buf = Vec::with_capacity(payer.encoded_len());
    payer.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(payer.account_id.as_ref().unwrap().data.to_vec()),
            value: Bytes::from(buf),
        },
        cf: USERS_COL_FAMILY,
        ttl: 0,
    }).await?;

    // Update payee
    let mut buf = Vec::with_capacity(payee.encoded_len());
    payee.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(payee.account_id.as_ref().unwrap().data.to_vec()),
            value: Bytes::from(buf),
        },
        cf: USERS_COL_FAMILY,
        ttl: 0,
    }).await?;

    Ok(())
}
