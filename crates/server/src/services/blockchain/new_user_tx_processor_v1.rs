// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;

use base::karma_coin::karma_coin_core_types::{Amount, Balance, CoinType, SignedTransaction};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use crate::services::db_config_service::USERS_COL_FAMILY;
use prost::Message;

/// Process a new user transaction
pub (crate) async fn _process_transaction(transaction: &SignedTransaction) -> Result<()> {

    let account_id = transaction.signer.as_ref().ok_or_else(|| anyhow!("missing account id in tx"))?;

    if (DatabaseService::read(ReadItem {
        key: Bytes::from(account_id.data.clone()),
        cf: USERS_COL_FAMILY
    }).await?).is_some() {
        return Err(anyhow!("account already on chain"));
    }

    transaction.validate(0)?;

    let new_user_tx = transaction.get_new_user_transaction_v1()?;

    // Get user provider info from tx but overwrite with data that should be added
    // according to the consensus rule
    let mut user = new_user_tx.user.ok_or_else(|| anyhow!("missing user data in tx"))?;

    let verification_evidence = new_user_tx.verify_number_response.ok_or_else(|| anyhow!("missing verifier data"))?;
    verification_evidence.verify_signature()?;

    // todo: this must come from config - new user tx gas fees
    let tx_fee_kcents = 1000;
    let signup_reward_kcents = 10^9;

    // todo: calc tx fee from user to validator

    user.nonce = 1;
    user.balances = vec![Balance {
        free: Some(Amount {
            value: signup_reward_kcents - tx_fee_kcents,
            coin_type: CoinType::Core as i32,
        }),
        reserved: None,
        misc_frozen: None,
        fee_frozen: None,
    }];

    // todo: figure out personality trait for joiner - brave?
    // ahead of the curve?
    user.trait_scores = vec![];

    // add user to db
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

    // index the transaction in the db

    // create transaction event and emit it

    // note that referral awards are handled in the payment tx processing logic and not here


    todo!()
}