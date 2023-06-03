// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{
    LEADER_BOARD_COL_FAMILY, MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY,
    USERS_NAMES_COL_FAMILY,
};
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::{
    ExecutionInfo, ExecutionResult, FeeType, SignedTransaction, TransactionBody, TransactionEvent,
    User,
};
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, DeleteItem, WriteItem};
use prost::Message;

#[derive(Debug, Clone)]
pub(crate) struct DeleteUserProcessingResponse {}

#[derive(Debug, Clone)]
pub(crate) struct DeleteUserProcessingError {
    pub(crate) execution_info: ExecutionInfo,
    pub(crate) error_message: String,
}

impl BlockChainService {
    /// Process a new user transaction - update ledger state, emit tx event
    /// This method will not add the tx to a block and is used as part of block creation flow
    pub(crate) async fn process_delete_user_transaction(
        &mut self,
        signed_transaction: &SignedTransaction,
        user: &User,
        event: &mut TransactionEvent,
    ) -> Result<DeleteUserProcessingResponse, DeleteUserProcessingError> {
        let account_id =
            signed_transaction
                .signer
                .as_ref()
                .ok_or_else(|| DeleteUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Missing verification evidence".into(),
                })?;

        let tx_hash = signed_transaction
            .get_hash()
            .map_err(|_| DeleteUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Missing verification evidence".into(),
            })?;

        // validate tx syntax, fields, signature, net_id before processing it
        // new user transaction should always have nonce of 0
        signed_transaction
            .validate()
            .await
            .map_err(|_| DeleteUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Invalid transaction data".into(),
            })?;

        let tx_body: TransactionBody =
            signed_transaction
                .get_body()
                .map_err(|_| DeleteUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Invalid transaction data".into(),
                })?;

        // Validate the Transaction object
        tx_body
            .validate(0)
            .await
            .map_err(|_| DeleteUserProcessingError {
                execution_info: ExecutionInfo::InvalidData,
                error_message: "Invalid transaction data".into(),
            })?;

        let _delete_user_tx =
            tx_body
                .get_delete_user_transaction_v1()
                .map_err(|_| DeleteUserProcessingError {
                    execution_info: ExecutionInfo::InvalidData,
                    error_message: "Invalid delete user tx data".into(),
                })?;

        // get tx_data to store on chain
        let mut tx_data = Vec::with_capacity(signed_transaction.encoded_len());
        info!(
            "binary transaction size: {}",
            signed_transaction.encoded_len()
        );

        signed_transaction
            .encode(&mut tx_data)
            .map_err(|_| DeleteUserProcessingError {
                execution_info: ExecutionInfo::InternalNodeError,
                error_message: "internal node error".into(),
            })?;

        //
        // end of tx data validation
        //
        ////////////////////////////////////

        // delete user account and update all indexes
        DatabaseService::delete(DeleteItem {
            key: Bytes::from(account_id.data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await
        .map_err(|_| DeleteUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "Can't delete user from db".into(),
        })?;

        DatabaseService::delete(DeleteItem {
            key: Bytes::from(user.user_name.as_bytes().to_vec()),
            cf: USERS_NAMES_COL_FAMILY,
        })
        .await
        .map_err(|_| DeleteUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "Can't delete user name from db".into(),
        })?;

        if let Some(mobile_number) = user.mobile_number.as_ref() {
            DatabaseService::delete(DeleteItem {
                key: Bytes::from(mobile_number.number.as_bytes().to_vec()),
                cf: MOBILE_NUMBERS_COL_FAMILY,
            })
            .await
            .map_err(|_| DeleteUserProcessingError {
                execution_info: ExecutionInfo::InternalNodeError,
                error_message: "cant delete user's number".into(),
            })?;
        }

        // ignore result
        let _res = DatabaseService::delete(DeleteItem {
            key: Bytes::from(account_id.data.clone()),
            cf: LEADER_BOARD_COL_FAMILY,
        })
        .await;

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
        .map_err(|_| DeleteUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "failed to write tx to chain".into(),
        })?;

        // index the transaction in the db by signer account id
        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(account_id.data.to_vec()),
        )
        .await
        .map_err(|_| DeleteUserProcessingError {
            execution_info: ExecutionInfo::InternalNodeError,
            error_message: "failed to index tx by account id".into(),
        })?;

        // 1 kCent protocol subsidy
        event.fee_type = FeeType::Mint as i32;
        event.fee = 1;
        event.result = ExecutionResult::Executed as i32;

        Ok(DeleteUserProcessingResponse {})
    }
}
