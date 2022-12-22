// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::collections::HashMap;
use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use prost::Message;
use base::karma_coin::karma_coin_core_types::*;
use base::karma_coin::karma_coin_core_types::TransactionType::NewUserV1;
use db::db_service::{DatabaseService, ReadItem};
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::mem_pool_service::{GetTransactions, MemPoolService, RemoveOnChainTransactions, RemoveTransactionByHash, RemoveTransactionsByHashes};
use crate::services::blockchain::{new_user_tx_processor, payment_tx_processor};
use crate::services::blockchain::stats::{get_stats};
use crate::services::db_config_service::USERS_COL_FAMILY;

#[message(result = "Result<Option<Block>>")]
pub(crate) struct ProcessTransactions;

/// Process transactions in the mem-pool and optionally create a block if one or more transactions were processed
#[async_trait::async_trait]
impl Handler<ProcessTransactions> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: ProcessTransactions,
    ) -> Result<Option<Block>> {

        let mem_pool = MemPoolService::from_registry().await?;
        let transactions_map = mem_pool.call(GetTransactions).await??;

        if transactions_map.is_empty() {
            info!("no txs in mempool to process");
            return Ok(None);
        }

        // remove from pool all transactions that are already on chain
        mem_pool.call(RemoveOnChainTransactions).await??;

        // get txs from pool post filtering
        let transactions_map = mem_pool.call(GetTransactions).await??;

        let stats = get_stats().await?;
        let height = stats.tip_height;
        let mut tx_hashes: Vec<Vec<u8>> = vec![];
        let mut block_event = BlockEvent::new(height+1);

        // signups indexed by mobile number
        let mut sign_ups: HashMap<Vec<u8>, SignedTransaction> = HashMap::new();

        for (tx_hash, tx) in transactions_map.iter() {
            if tx.get_tx_type()? != NewUserV1 {
                // here we only process new user transactions
                continue;
            }

            let mut event = TransactionEvent {
                timestamp: Utc::now().timestamp_millis() as u64,
                height: height +1,
                transaction: Some(tx.clone()),
                transaction_hash: tx_hash.to_vec(),
                result: ExecutionResult::Executed as i32,
                error_message: "".to_string(),
                fee_type: FeeType::Mint as i32,
            };

            match new_user_tx_processor::process_transaction(tx).await {
                Ok(mobile_number) => {
                    info!("new user transaction processed: {:?}", event);
                    tx_hashes.push(tx_hash.to_vec());
                    block_event.total_signups += 1;
                    block_event.add_fee(event.transaction.as_ref().unwrap().fee.as_ref().unwrap().value);
                    block_event.add_transaction_event(event);

                    // update new signups map - used for referrals and payments
                    sign_ups.insert(mobile_number.as_bytes().to_vec(), tx.clone());

                },
                Err(e) => {
                    error!("Failed to process new user transaction: {:?}", e);
                    event.result = ExecutionResult::Invalid as i32;
                    event.error_message = e.to_string();
                }
            }

            // remove the processed tx from the pool
            mem_pool.call(RemoveTransactionByHash(tx_hash.to_vec())).await??;
        }

        // process other transactions types
        for (tx_hash, tx) in transactions_map.iter() {
            let tx_type = tx.get_tx_type()?;
            if tx_type == NewUserV1 {
                // Skip any new user txs
                continue;
            }

            // Get User from chain and reject tx if user doesn't exist
            let mut user = match DatabaseService::read(ReadItem {
                key: Bytes::from(tx.signer.as_ref().unwrap().data.clone()),
                cf: USERS_COL_FAMILY
            }).await? {
                Some(data) => {
                   User::decode(data.0.as_ref())?
                },
                None => {
                    info!("Payee user not found on chain - removing tx from mempool");
                    mem_pool.call(RemoveTransactionByHash(tx_hash.to_vec())).await??;
                    continue;
                }
            };

            match tx_type {
                TransactionType::PaymentV1 => {
                    if let Some(mut payee) = payment_tx_processor::get_payee_user(tx).await? {
                        let mut event = TransactionEvent {
                            timestamp: Utc::now().timestamp_millis() as u64,
                            height: height + 1,
                            transaction: Some(tx.clone()),
                            transaction_hash: tx_hash.to_vec(),
                            result: ExecutionResult::Executed as i32,
                            error_message: "".to_string(),
                            fee_type: FeeType::Mint as i32,
                        };

                        match payment_tx_processor::process_transaction(tx,  &mut user, &mut payee, &sign_ups).await {
                                Ok(_) => {
                                    event.result = ExecutionResult::Executed as i32;
                                    info!("payment transaction processed: {:?}", event);
                                    tx_hashes.push(tx_hash.to_vec());
                                    block_event.total_payments += 1;
                                    block_event.add_fee(event.transaction.as_ref().unwrap().fee.as_ref().unwrap().value);
                                    block_event.add_transaction_event(event);
                                },
                                Err(e) => {
                                    info!("payment transaction failed: {:?}", e);
                                    event.result = ExecutionResult::Invalid as i32;
                                    event.error_message = e.to_string();

                                    // add the invalid tx to the block event
                                    // todo: block producer should get fee when possible from an invalid tx
                                    block_event.add_transaction_event(event);
                                }
                            }
                        }
                    else {
                        info!("Payee user not found on chain - keep this tx in the mem pool");
                        continue;
                    }
                },
                TransactionType::UpdateUserV1 => {
                    todo!("process update user transaction");
                },
                _ => {
                    // ignore other transaction types
                }
            }
        }

        if tx_hashes.is_empty() {
            info!("no txs to add to the block - skip block creation....");
            return Ok(None);
        }

        // create the block
        let block = BlockChainService::create_block(&tx_hashes,
                                                    stats,
                                                    block_event,
                                                    height + 1,
                                                    self.id_key_pair.as_ref().unwrap()).await?;

        mem_pool.call(RemoveTransactionsByHashes(tx_hashes)).await??;
        Ok(Some(block))
    }
}
