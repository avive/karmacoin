// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::base::signed_trait::SignedTrait;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY,
};
use anyhow::Result;
use base::genesis_config_service::SIGNUP_CHAR_TRAIT_ID;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::{
    CommunityMembership, ExecutionInfo, ExecutionResult, FeeType, SignedTransaction, TraitScore,
    TransactionBody, TransactionEvent, TransactionType, User,
};
use bytes::Bytes;
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

        let tx_body: TransactionBody =
            signed_transaction
                .get_body()
                .map_err(|_| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Invalid transaction data".into(),
                })?;

        // Validate the Transaction object
        tx_body
            .validate(0)
            .await
            .map_err(|_| NewUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Invalid transaction data".into(),
            })?;

        let tx_fee_amount = tx_body.fee;
        let new_user_tx =
            tx_body
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
        // todo: verify verifier is valid according to consensus rules and genesis config
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
                error_message: "account id must match account id in verification data".into(),
            });
        }

        // we allow creation of a new account with a number that already has a different account on chain
        // to handle the case where user lost access to his old account private key.
        // a new account will be created with the user requested accountId and associated with this number
        // the old account is still on-chain and is accessible via transactions that use account id
        /*
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(mobile_number.number.as_bytes().to_vec()),
            cf: MOBILE_NUMBERS_COL_FAMILY,
        })
        .await
        .map_err(|_| NewUserProcessingError {
            execution_info: ExecutionInfo::InvalidData,
            error_message: "internal node error".into(),
        })? {
            let existing_user =
                User::decode(data.0.as_ref()).map_err(|_| NewUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "internal node error".into(),
                })?;

            info!(
                "There's already a user with account id {} for this mobile number",
                short_hex_string(existing_user.account_id.as_ref().unwrap().data.as_ref())
            );

            return Err(NewUserProcessingError {
                execution_info: ExecutionInfo::AccountAlreadyExists,
                error_message: "account already exists for the provided mobile number. consider updating your existing account".into(),
            });
        } else {
            info!('there is no existing user account with th)
        }*/

        info!(
            "new user transaction for {}, {}, accountId: {}",
            verification_evidence.requested_user_name,
            mobile_number.number,
            short_hex_string(account_id.data.as_ref())
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
                error_message: "there's already an onchain account for the provided account id"
                    .into(),
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
                error_message: "there's already an account with requested user name".into(),
            });
        }

        //
        // end of user data validation part
        //
        /////////////////////////////////////////////////////////////////////////

        // signup char trait assignment to new user
        let sign_up_trait_score = TraitScore {
            trait_id: SIGNUP_CHAR_TRAIT_ID,
            score: 1,
            community_id: 0,
        };

        let mut community_memberships: Vec<CommunityMembership> = vec![];

        // hack to set admin for specific numbers in test community
        // should be handeled by admin api / sudo
        if mobile_number.number == "+972549805380"
        /*|| mobile_number.number == "+972549805381"*/
        {
            community_memberships.push(CommunityMembership {
                community_id: 1,
                karma_score: 1, // initial community karma score is 1 for joining
                is_admin: true,
            });
        }

        let mut new_user = User {
            account_id: Some(account_id.clone()),
            nonce: 1, // signup tx nonce is 1, so the next tx nonce should be 2
            user_name: verification_evidence.requested_user_name.clone(),
            mobile_number: Some(mobile_number.clone()),
            balance: 0,
            trait_scores: vec![sign_up_trait_score],
            pre_keys: vec![],
            karma_score: 1, // initial karma score is 1 for getting the signup trait score
            community_memberships,
        };

        let apply_subsidy = tokenomics
            .should_subsidise_transaction_fee(0, tx_fee_amount, TransactionType::NewUserV1)
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

        let user_tx_fee_amount = if apply_subsidy { 0 } else { tx_fee_amount };

        if !apply_subsidy && tx_fee_amount >= signup_reward_amount {
            // invalid tx - tx fee is higher than the block award and no tx fee subsidy is applied
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

        new_user.balance += signup_reward_amount - user_tx_fee_amount;

        info!("new user balance: {}", new_user.balance);

        // add the new user to db

        // todo: update existing user if it exists - this will happen for a block producer or a verifier

        let mut buf = Vec::with_capacity(new_user.encoded_len());
        new_user
            .encode(&mut buf)
            .map_err(|_| NewUserProcessingError {
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

        info!("added user to db");

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

        // update the txs event
        event.fee_type = fee_type as i32;
        event.fee = tx_fee_amount;
        event.signup_reward = signup_reward_amount;
        event.result = ExecutionResult::Executed as i32;

        Ok(NewUserProcessingResponse {
            mobile_number: mobile_number.number.clone(),
        })
    }
}
