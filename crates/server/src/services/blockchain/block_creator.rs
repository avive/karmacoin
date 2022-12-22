// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::collections::HashMap;
use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_core_types::*;
use base::karma_coin::karma_coin_core_types::TransactionType::NewUserV1;
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use db::types::IntDbKey;
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::mem_pool_service::{GetTransactions, MemPoolService, RemoveOnChainTransactions, RemoveTransactionByHash};
use crate::services::blockchain::{new_user_tx_processor, payment_tx_processor};
use crate::services::blockchain::stats::{get_stats, write_stats};
use crate::services::db_config_service::{BLOCK_EVENTS_COL_FAMILY, BLOCKS_COL_FAMILY, USERS_COL_FAMILY};

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

            match new_user_tx_processor::process_transaction(tx, height + 1).await {
                Ok(result) => {
                    let event = result.event;
                    info!("new user transaction processed: {:?}", event);
                    tx_hashes.push(tx_hash.to_vec());
                    block_event.total_signups += 1;
                    block_event.add_fee(event.transaction.as_ref().unwrap().fee.as_ref().unwrap().value);
                    block_event.add_transaction_event(event);

                    // update new signups map - used for referrals and payments
                    sign_ups.insert(result.mobile_number.as_bytes().to_vec(), tx.clone());

                },
                Err(e) => {
                    error!("Failed to process new user transaction: {:?}", e);
                }
            }

            // remove the processed tx from the pool
            mem_pool.call(RemoveTransactionByHash(tx_hash.to_vec())).await??;
        }

        // process other transactions types
        for (tx_hash, tx) in transactions_map.iter() {
            let tx_type = tx.get_tx_type()?;
            if tx_type == NewUserV1 {
                // we don't process new user transactions here
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
                            match payment_tx_processor::process_transaction(tx, height + 1, &mut user, &mut payee, &sign_ups).await {
                                Ok(event) => {
                                    info!("payment transaction processed: {:?}", event);
                                    tx_hashes.push(tx_hash.to_vec());
                                    block_event.total_payments += 1;
                                    block_event.add_fee(event.transaction.as_ref().unwrap().fee.as_ref().unwrap().value);
                                    block_event.add_transaction_event(event);
                                },
                                Err(e) => {
                                    info!("payment transaction failed: {:?}", e);
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
            info!("no txs were processed - skip block creation");
            return Ok(None);
        }

        // create the block
        let block = BlockChainService::create_block_helper(&tx_hashes,
                                                     stats,
                                                     block_event,
                                                     height + 1,
                                                     self.id_key_pair.as_ref().unwrap()).await?;

        // remove all transactions added to the block from the mem-pool
        for tx_hash in tx_hashes.iter() {
            mem_pool.call(RemoveTransactionByHash(tx_hash.to_vec())).await??;
        }

        Ok(Some(block))
    }
}

/// BlockchainService block creation implementation
impl BlockChainService {

    /// Create a block with the provided txs hashes at a given height
    /// Internal help method
    async fn create_block_helper(transactions_hashes: &Vec<Vec<u8>>,
                                 stats: BlockchainStats,
                                 mut block_event: BlockEvent,
                                 height: u64,
                                 key_pair: &KeyPair
    ) -> Result<Block> {
        let mut block = Block {
            time: chrono::Utc::now().timestamp_millis() as u64,
            author: None,
            height,
            transactions_hashes: transactions_hashes.clone(),
            fees: Some(block_event.total_fees.as_ref().unwrap().clone()),
            signature: None,
            prev_block_digest: vec![],
            digest: vec![],
        };

        // Set previous block hash to the hash of the previous block
        if height > 0 {
            let Some(prev_block_data) = DatabaseService::read(ReadItem {
                key: IntDbKey::from(height).0,
                cf: BLOCKS_COL_FAMILY
            }).await? else {
                return Err(anyhow::anyhow!("Failed to read previous block"));
            };

            let prev_block = Block::decode(prev_block_data.0)?;
            block.prev_block_digest = prev_block.digest;
        } else {
            info!("creating genesis block - no prev one");
        };

        // compute block hash and set it
        block.digest = block.get_hash()?.to_vec();

        // sign the block
        block.sign(&key_pair.to_ed2559_kaypair())?;

        // insert the block to the db
        let mut buf = Vec::with_capacity(block.encoded_len());
        block.encode(&mut buf)?;

        // Write the block to the db
        DatabaseService::write(
            WriteItem {
                data: DataItem {
                    key: IntDbKey::from(height).0,
                    value: Bytes::from(buf)
                },
                cf: BLOCKS_COL_FAMILY,
                ttl: 0,
            }).await?;

        // Update and persist block event
        block_event.block_hash = block.digest.clone();

        let mut buf = Vec::with_capacity(block_event.encoded_len());
        block_event.encode(&mut buf)?;
        DatabaseService::write(
            WriteItem {
                data: DataItem {
                    key: IntDbKey::from(height).0,
                    value: Bytes::from(buf)
                },
                cf: BLOCK_EVENTS_COL_FAMILY,
                ttl: 0,
            }).await?;

        // Update blockchain global stats and persist
        BlockChainService::update_blockchain_stats(stats, &block_event, &block).await?;

        Ok(block)
    }

    /// Update blockchain stats with new block data and store in db
    async fn update_blockchain_stats(mut stats: BlockchainStats, block_event: &BlockEvent, block: &Block) -> Result<()> {

        stats.tip_height += 1;
        stats.users += block_event.total_signups;
        stats.fees.as_mut().unwrap().value += block_event.total_fees.as_ref().unwrap().value;
        stats.signup_rewards.as_mut().unwrap().value += block_event.total_signup_rewards.as_ref().unwrap().value;
        stats.referral_rewards.as_mut().unwrap().value += block_event.total_referral_rewards.as_ref().unwrap().value;
        stats.transactions += block.transactions_hashes.len() as u64;
        stats.last_block_time = block.time;
        stats.payments +=  block_event.total_payments;

        write_stats(stats).await
    }
}



