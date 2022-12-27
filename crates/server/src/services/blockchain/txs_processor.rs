// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::mem_pool_service::{
    GetTransactions, MemPoolService, RemoveOldTransactions, RemoveOnChainTransactions,
    RemoveTransactionByHash, RemoveTransactionsByHashes,
};
use crate::services::blockchain::stats::get_stats;
use crate::services::blockchain::{
    new_user_tx_processor, payment_tx_processor, update_tx_processor,
};
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::TransactionType::NewUserV1;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use std::collections::HashMap;

use crate::services::blockchain::tokenomics::Tokenomics;
use xactor::*;

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

        // remove from pool all transactions that are already on chain
        mem_pool.call(RemoveOnChainTransactions).await??;

        // get all pending txs from mem-pool
        let transactions_map = mem_pool.call(GetTransactions).await??;

        if transactions_map.is_empty() {
            info!("mem pool empty");
            return Ok(None);
        }

        // get current blockchain stats and tokenomics
        let stats = get_stats().await?;
        let tokenomics = Tokenomics {
            stats: stats.clone(),
        };
        let height = stats.tip_height + 1;
        let mut tx_hashes: Vec<Vec<u8>> = vec![];

        // the block event for the new block
        let mut block_event = BlockEvent::new(height + 1);

        // new signups txs indexed by mobile number - used for referral reward calculations
        let mut sign_ups: HashMap<Vec<u8>, SignedTransaction> = HashMap::new();

        for (tx_hash, tx) in transactions_map.iter() {
            if tx.get_tx_type()? != NewUserV1 {
                // in this loop we only process new user transactions
                continue;
            }

            // the transaction event for the new user transaction
            let mut tx_event = TransactionEvent::new(height, tx, tx_hash);

            match new_user_tx_processor::process_transaction(tx, &tokenomics, &mut tx_event).await {
                Ok(res) => {
                    info!("new user transaction processed: {:?}", tx_event);
                    tx_hashes.push(tx_hash.to_vec());
                    block_event.signups_count += 1;
                    block_event.add_fee(tx_event.transaction.as_ref().unwrap().fee);
                    block_event.add_transaction_event(tx_event.clone());
                    // update new signups map - used for referrals and payments
                    sign_ups.insert(res.mobile_number.as_bytes().to_vec(), tx.clone());
                }
                Err(e) => {
                    error!("Failed to process new user transaction: {:?}", e);
                    tx_event.result = ExecutionResult::Invalid as i32;
                    tx_event.error_message = e.to_string();
                }
            }

            // remove the processed tx from the pool
            mem_pool
                .call(RemoveTransactionByHash(tx_hash.to_vec()))
                .await??;

            // emit the tx event
            BlockChainService::emit_tx_event(tx_event).await?;
        }

        // process other transactions types
        for (tx_hash, tx) in mem_pool.call(GetTransactions).await??.iter() {
            let tx_type = tx.get_tx_type()?;
            let mut tx_event = TransactionEvent::new(height, tx, tx_hash);

            // Get tx issuer user from chain and reject tx if it doesn't exist
            let mut user = match DatabaseService::read(ReadItem {
                key: Bytes::from(tx.signer.as_ref().unwrap().data.clone()),
                cf: USERS_COL_FAMILY,
            })
            .await?
            {
                Some(data) => User::decode(data.0.as_ref())?,
                None => {
                    info!("Tx signer user not found on chain - removing tx from mempool");
                    mem_pool
                        .call(RemoveTransactionByHash(tx_hash.to_vec()))
                        .await??;
                    tx_event.result = ExecutionResult::Invalid as i32;
                    tx_event.error_message =
                        "Tx signer user not found on chain - discarding tx".to_string();

                    BlockChainService::emit_tx_event(tx_event.clone()).await?;
                    continue;
                }
            };

            match tx_type {
                TransactionType::PaymentV1 => {
                    if let Some(mut payee) = payment_tx_processor::get_payee_user(tx).await? {
                        match payment_tx_processor::process_transaction(
                            tx,
                            &mut user,
                            &mut payee,
                            &mut sign_ups,
                            &tokenomics,
                            &mut tx_event,
                        )
                        .await
                        {
                            Ok(_) => {
                                info!("payment transaction processed: {:?}", tx_event);
                                tx_hashes.push(tx_hash.to_vec());
                                block_event.payments_count += 1;
                                block_event.add_fee(tx_event.transaction.as_ref().unwrap().fee);
                                block_event.add_transaction_event(tx_event.clone());
                            }
                            Err(e) => {
                                info!("payment transaction failed: {:?}", e);
                                tx_event.result = ExecutionResult::Invalid as i32;
                                tx_event.error_message = e.to_string();
                            }
                        }

                        BlockChainService::emit_tx_event(tx_event).await?;
                    } else {
                        info!("Payee user not found on chain - keeping this tx in the mem pool for later processing...");
                        continue;
                    }
                }
                TransactionType::UpdateUserV1 => {
                    match update_tx_processor::process_transaction(tx, &tokenomics, &mut tx_event)
                        .await
                    {
                        Ok(_) => {
                            info!("update user transaction processed: {:?}", tx_event);
                            tx_hashes.push(tx_hash.to_vec());
                            block_event.add_fee(tx_event.transaction.as_ref().unwrap().fee);
                            block_event.add_transaction_event(tx_event.clone());
                        }
                        Err(e) => {
                            error!("Failed to process update user transaction: {:?}", e);
                            tx_event.result = ExecutionResult::Invalid as i32;
                            tx_event.error_message = e.to_string();
                        }
                    }
                    BlockChainService::emit_tx_event(tx_event).await?;
                }
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
        let block = BlockChainService::create_block(
            &tx_hashes,
            stats,
            &tokenomics,
            block_event,
            height + 1,
            self.id_key_pair.as_ref().unwrap(),
        )
        .await?;

        // remove processed txs from the mem pool
        mem_pool
            .call(RemoveTransactionsByHashes(tx_hashes))
            .await??;

        // remove old txs from the mem pool
        mem_pool.call(RemoveOldTransactions).await??;
        Ok(Some(block))
    }
}
