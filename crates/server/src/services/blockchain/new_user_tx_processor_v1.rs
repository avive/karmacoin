// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;

use base::karma_coin::karma_coin_core_types::{Amount, Balance, CoinType, ExecutionResult, FeeType, SignedTransaction, TransactionEvent};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, RESERVED_NICKS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY};
use prost::Message;
use base::blockchain_config_service::{BlockchainConfigService, DEF_TX_FEE_KEY, SIGNUP_REWARD_KEY};

/// Process a new user transaction - update ledger state, emit tx event
/// This method will not add the tx to a block nor index it
///
pub (crate) async fn process_transaction(
    transaction: &SignedTransaction,
    block_height: u64) -> Result<TransactionEvent> {

    let account_id = transaction.signer.as_ref().ok_or_else(|| anyhow!("missing account id in tx"))?;

    if (DatabaseService::read(ReadItem {
        key: Bytes::from(account_id.data.clone()),
        cf: USERS_COL_FAMILY
    }).await?).is_some() {
        return Err(anyhow!("account already on chain"));
    }

    transaction.validate(0)?;

    let new_user_tx = transaction.get_new_user_transaction_v1()?;

    let mut user = new_user_tx.user.ok_or_else(|| anyhow!("missing user data in tx"))?;
    let verification_evidence = new_user_tx.verify_number_response.ok_or_else(|| anyhow!("missing verifier data"))?;

    // verify evidence signature
    // todo: verify verifier is valid according to consensus rules
    // and genesis config
    verification_evidence.verify_signature()?;

    // validate verification evidence with user provided data
    let user_mobile_number = user.mobile_number.as_ref().ok_or_else(|| anyhow!("missing mobile number"))?;
    let evidence_mobile_number = verification_evidence.mobile_number.ok_or_else(|| anyhow!("missing mobile number in verifier data"))?;
    let user_account_id =  user.account_id.as_ref().ok_or_else(|| anyhow!("missing account id in user data"))?;
    let evidence_account_id = verification_evidence.account_id.ok_or_else(|| anyhow!("missing account id in verifier data"))?;

    if user_account_id.data != evidence_account_id.data {
        return Err(anyhow!("account id mismatch"));
    }

    if user.user_name != verification_evidence.nickname {
        return Err(anyhow!("nickname mismatch"));
    }

    if user_mobile_number.number != evidence_mobile_number.number {
        return Err(anyhow!("mobile number mismatch"));
    }

    // Create the user and update its data
    let tx_fee_k_cents = BlockchainConfigService::get_u64(DEF_TX_FEE_KEY.into()).await?.unwrap();
    let signup_reward_k_cents = BlockchainConfigService::get_u64(SIGNUP_REWARD_KEY.into()).await?.unwrap();

    user.nonce = 1;
    user.balances = vec![Balance {
        free: Some(Amount {
            value: signup_reward_k_cents - tx_fee_k_cents,
            coin_type: CoinType::Core as i32,
        }),
        reserved: None,
        misc_frozen: None,
        fee_frozen: None,
    }];

    // todo: figure out personality trait for joiner - brave? ahead of the curve?
    user.trait_scores = vec![];

    // add the new user to db
    let mut buf = Vec::with_capacity(user.encoded_len());
    user.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(account_id.data.to_vec()),
            value: Bytes::from(buf),
         },
        cf: USERS_COL_FAMILY,
        ttl: 0,
    }).await?;

    // update nickname index
    DatabaseService::write(WriteItem {
        data: DataItem { key: Bytes::from(user.user_name.as_bytes().to_vec()), value: Bytes::from(account_id.data.to_vec()) },
        cf: RESERVED_NICKS_COL_FAMILY,
        ttl: 0,
    }).await?;

    // update mobile numbers index
    DatabaseService::write(WriteItem {
        data: DataItem { key: Bytes::from(user_mobile_number.number.as_bytes().to_vec()), value: Bytes::from(account_id.data.to_vec()) },
        cf: MOBILE_NUMBERS_COL_FAMILY,
        ttl: 0,
    }).await?;

    let mut tx_data = Vec::with_capacity(transaction.encoded_len());
    transaction.encode(&mut tx_data)?;

    let tx_hash = transaction.get_hash()?;

    // index the transaction in the db
    DatabaseService::write(WriteItem {
        data: DataItem { key: tx_hash.clone(),
            value: Bytes::from(tx_data) },
        cf: TRANSACTIONS_COL_FAMILY,
        ttl: 0,
    }).await?;

    // todo: transfer the tx fee to the local block producer (this node)

    // note that referral awards are handled in the payment tx processing logic and not here

    Ok(TransactionEvent {
        height: block_height,
        transaction: Some(transaction.clone()),
        transaction_hash: tx_hash.as_ref().to_vec(),
        result: ExecutionResult::Executed as i32,
        fee_type: FeeType::Mint as i32,
    })
}