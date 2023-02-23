// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;

use crate::base::signed_trait::SignedTrait;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY,
};
use base::genesis_config_service::SIGNUP_CHAR_TRAIT_ID;
use base::karma_coin::karma_coin_core_types::{
    ExecutionInfo, ExecutionResult, FeeType, SignedTransaction, TraitScore, TransactionBody,
    TransactionEvent, TransactionType, User,
};
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;

#[derive(Debug, Clone)]
pub(crate) struct NewUserProcessingResponse {
    pub(crate) mobile_number: String,
}

#[derive(Debug, Clone)]
pub(crate) struct NewUserProcessingError {
    pub(crate) execution_info: ExecutionInfo,
    pub(crate) error_message: String,
}

impl BlockChainService {
    /// Process a new user transaction - update ledger state, emit tx event
    /// This method will not add the tx to a block and is used as part of block creation flow
    pub(crate) async fn process_new_user_transaction(
        &mut self,
        signed_transaction: &SignedTransaction,
        tokenomics: &Tokenomics,
        event: &mut TransactionEvent,
    ) -> Result<NewUserProcessingResponse, NewUserProcessingError> {
        let account_id =
            signed_transaction
                .signer
                .as_ref()
                .ok_or_else(|| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Missing verification evidence".into(),
                })?;

        let tx_hash = signed_transaction
            .get_hash()
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Missing verification evidence".into(),
            })?;

        // validate tx syntax, fields, signature, net_id before processing it
        // new user transaction should always have nonce of 0
        signed_transaction
            .validate()
            .await
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Invalid transaction data".into(),
            })?;

        let tx: TransactionBody =
            signed_transaction
                .get_body()
                .map_err(|_| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Invalid transaction data".into(),
                })?;

        // Validate the Transaction object
        tx.validate(0).await.map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InvalidData,
            error_message: "Invalid transaction data".into(),
        })?;

        let tx_fee = tx.fee;
        let new_user_tx = tx
            .get_new_user_transaction_v1()
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Invalid new user tx data".into(),
            })?;

        let verification_evidence =
            new_user_tx
                .verify_number_response
                .ok_or_else(|| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "missing verification evidence".into(),
                })?;

        // verify evidence signature
        // todo: verify verifier is valid according to consensus rules
        // and genesis config
        verification_evidence
            .verify_signature()
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "invalid verification signature".into(),
            })?;

        let mobile_number =
            verification_evidence
                .mobile_number
                .ok_or_else(|| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "missing mobile number in evidence".into(),
                })?;

        let evidence_account_id =
            verification_evidence
                .account_id
                .ok_or_else(|| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "missing account id in evidence".into(),
                })?;

        if account_id.data != evidence_account_id.data {
            return Err(NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "account id must match account id in verification data mismatch"
                    .into(),
            });
        }

        info!(
            "new user transaction for {} - {}",
            verification_evidence.requested_user_name, mobile_number.number
        );

        // Check user account id is not already on chain
        if (DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InvalidData,
            error_message: "internal node error".into(),
        }))?
        .is_some()
        {
            return Err(NewUserProcessingError {
                execution_info: ExecutionInfo::AccountAlreadyExists,
                error_message: "There is already an account the provided account id".into(),
            });
        }

        // Check requested user name is not already on chain
        if (DatabaseService::read(ReadItem {
            key: Bytes::from(verification_evidence.requested_user_name.clone()),
            cf: USERS_NAMES_COL_FAMILY,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InvalidData,
            error_message: "internal node error".into(),
        }))?
        .is_some()
        {
            return Err(NewUserProcessingError {
                execution_info: ExecutionInfo::NicknameNotAvailable,
                error_message: "There is already an account with requested user name".into(),
            });
        }

        let sign_up_trait_score = TraitScore {
            trait_id: SIGNUP_CHAR_TRAIT_ID,
            score: 1,
        };

        let mut user = User {
            account_id: Some(account_id.clone()),
            nonce: 1,
            user_name: verification_evidence.requested_user_name.clone(),
            mobile_number: Some(mobile_number.clone()),
            balance: 0,
            trait_scores: vec![sign_up_trait_score],
            pre_keys: vec![],
            karma_score: 1,
        };

        let apply_subsidy = tokenomics
            .should_subsidise_transaction_fee(0, tx_fee, TransactionType::NewUserV1)
            .await
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InternalNodeError,
                error_message: "internal node error".into(),
            })?;

        let signup_reward_amount =
            tokenomics
                .get_signup_reward_amount()
                .await
                .map_err(|_| NewUserProcessingError {
                    execution_info: ExecutionInfo::InternalNodeError,
                    error_message: "internal node error".into(),
                })?;

        info!("Current signup reward amount: {}", signup_reward_amount);

        let user_tx_fee = if apply_subsidy { 0 } else { tx_fee };

        if !apply_subsidy && tx_fee >= signup_reward_amount {
            // invalid tx - tx fee is higher than the block award
            return Err(NewUserProcessingError {
                execution_info: ExecutionInfo::TxFeeTooLow,
                error_message:
                    "Transaction fee is greater than signup reward and no tx fee subsidy is applied"
                        .into(),
            });
        }

        let fee_type = if apply_subsidy {
            FeeType::Mint
        } else {
            FeeType::User
        };

        user.balance += signup_reward_amount - user_tx_fee;

        info!("new user balance: {}", user.balance);

        // todo: figure out personality trait for joiner - brave? ahead of the curve?
        user.trait_scores = vec![];

        // add the new user to db

        // todo: update existing user if it exists - this will happen for a block producer or a verifier

        let mut buf = Vec::with_capacity(user.encoded_len());
        user.encode(&mut buf).map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "internal node error".into(),
        })?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(account_id.data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "internal node error".into(),
        })?;

        // update user name index
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(
                    verification_evidence
                        .requested_user_name
                        .as_bytes()
                        .to_vec(),
                ),
                value: Bytes::from(account_id.data.to_vec()),
            },
            cf: USERS_NAMES_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InvalidData,
            error_message: "internal node error".into(),
        })?;

        // update mobile numbers index
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(mobile_number.number.as_bytes().to_vec()),
                value: Bytes::from(account_id.data.to_vec()),
            },
            cf: MOBILE_NUMBERS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "internal node error".into(),
        })?;

        let mut tx_data = Vec::with_capacity(signed_transaction.encoded_len());
        info!(
            "binary transaction size: {}",
            signed_transaction.encoded_len()
        );

        signed_transaction
            .encode(&mut tx_data)
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InternalNodeError,
                error_message: "internal node error".into(),
            })?;

        // index the transaction in the db by hash
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: tx_hash.clone(),
                value: Bytes::from(tx_data),
            },
            cf: TRANSACTIONS_COL_FAMILY,
            ttl: 0,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "internal node error".into(),
        })?;

        // index the transaction in the db by signer account id
        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(account_id.data.to_vec()),
        )
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "internal node error".into(),
        })?;

        event.fee_type = fee_type as i32;
        event.fee = tx_fee;
        event.signup_reward = signup_reward_amount;
        event.result = ExecutionResult::Executed as i32;

        Ok(NewUserProcessingResponse {
            mobile_number: mobile_number.number.clone(),
        })
    }
}
