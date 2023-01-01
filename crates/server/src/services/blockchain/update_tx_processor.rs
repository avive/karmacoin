// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use bytes::Bytes;

use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, NICKS_COL_FAMILY, RESERVED_NICKS_COL_FAMILY,
    TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY,
};
use base::karma_coin::karma_coin_core_types::ExecutionInfo::{
    InvalidData, NicknameInvalid, NicknameNotAvailable,
};
use base::karma_coin::karma_coin_core_types::{
    ExecutionResult, FeeType, SignedTransaction, TransactionEvent, User,
};
use base::signed_trait::SignedTrait;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;

/// Helper function - update user's nickname
async fn update_nickname(
    user: &mut User,
    nickname: String,
    event: &mut TransactionEvent,
) -> Result<()> {
    let nick_name_key = Bytes::from(nickname.as_bytes().to_vec());
    let account_id = user.account_id.as_ref().unwrap();
    // verify that the requested nickname not registered to another user

    if (DatabaseService::read(ReadItem {
        key: nick_name_key.clone(),
        cf: RESERVED_NICKS_COL_FAMILY,
    })
    .await?)
        .is_some()
    {
        event.info = NicknameNotAvailable as i32;
        return Ok(());
    }

    if (DatabaseService::read(ReadItem {
        key: nick_name_key.clone(),
        cf: NICKS_COL_FAMILY,
    })
    .await?)
        .is_some()
    {
        event.info = NicknameNotAvailable as i32;
        return Ok(());
    }

    // update user's nickname
    user.user_name = nickname.clone();
    let mut buf = Vec::with_capacity(user.encoded_len());
    user.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(account_id.data.clone()),
            value: Bytes::from(buf),
        },
        cf: USERS_COL_FAMILY,
        ttl: 0,
    })
    .await?;

    // update nickname index
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: nick_name_key,
            value: Bytes::from(account_id.data.to_vec()),
        },
        cf: NICKS_COL_FAMILY,
        ttl: 0,
    })
    .await
}

/// Process a user update transaction
pub(crate) async fn process_transaction(
    transaction: &SignedTransaction,
    tokenomics: &Tokenomics,
    event: &mut TransactionEvent,
) -> Result<()> {
    let account_id = transaction
        .signer
        .as_ref()
        .ok_or_else(|| anyhow!("missing account id in tx"))?;
    let tx_hash = transaction.get_hash()?;

    // validate tx syntax, fields, signature, net_id before processing it
    transaction.validate(0).await?;

    // Check user account id is not already on chain
    let user_data = DatabaseService::read(ReadItem {
        key: Bytes::from(account_id.data.clone()),
        cf: USERS_COL_FAMILY,
    })
    .await?;

    if user_data.is_none() {
        return Err(anyhow!("user account not found on chain"));
    }

    let mut user = User::decode(user_data.unwrap().0.as_ref())?;

    // check tx fee
    let tx_fee = transaction.fee;
    let apply_subsidy = tokenomics
        .should_subsidise_transaction_fee(0, tx_fee)
        .await?;

    let fee_type = if apply_subsidy {
        FeeType::Mint
    } else {
        FeeType::User
    };

    if !apply_subsidy && tx_fee >= user.balance {
        // invalid tx - tx fee is higher than user balance
        return Err(anyhow!(
            "tx fee is greater than user balance no tx fee subsidy is applied"
        ));
    }

    // apply tx fee to user balance no subsidy is applied
    if !apply_subsidy {
        user.balance -= tx_fee;
        let mut buf = Vec::with_capacity(user.encoded_len());
        user.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account_id.data.clone()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;
    }

    event.fee = tx_fee;
    event.fee_type = fee_type as i32;
    event.result = ExecutionResult::Executed as i32;

    let update_user_tx = transaction.get_update_user_transaction_v1()?;
    update_user_tx.verify_syntax()?;

    let requested_nickname = update_user_tx.nickname;
    if requested_nickname.is_empty() && update_user_tx.verify_number_response == None {
        event.info = NicknameInvalid as i32;
        return Ok(());
    }

    // handle nickname update request...

    if user.user_name != requested_nickname {
        update_nickname(&mut user, requested_nickname, event).await?;
    }

    // handle mobile number update, if requested

    if update_user_tx.verify_number_response.is_none() {
        return Ok(());
    }

    let evidence = update_user_tx.verify_number_response.unwrap();
    if evidence.verify_signature().is_err() {
        event.info = InvalidData as i32;
        return Ok(());
    }

    // todo: verify evidence fields are valid

    let new_mobile_number = update_user_tx.mobile_number.unwrap();
    let verified_number = evidence.mobile_number.unwrap();

    if new_mobile_number.number != verified_number.number {
        event.info = InvalidData as i32;
        return Ok(());
    }

    let evidence_account_id = evidence
        .account_id
        .ok_or_else(|| anyhow!("missing account id in verifier data"))?;

    if account_id.data != evidence_account_id.data {
        event.info = InvalidData as i32;
        return Ok(());
    }

    // update user's mobile number
    user.mobile_number = Some(new_mobile_number.clone());
    let mut buf = Vec::with_capacity(user.encoded_len());
    user.encode(&mut buf)?;
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(account_id.data.clone()),
            value: Bytes::from(buf),
        },
        cf: USERS_COL_FAMILY,
        ttl: 0,
    })
    .await?;

    // update mobile numbers index
    DatabaseService::write(WriteItem {
        data: DataItem {
            key: Bytes::from(new_mobile_number.number.as_bytes().to_vec()),
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

    Ok(())
}
