// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::mem_pool_service::{
    GetTransactions, MemPoolService, RemoveOldTransactions, RemoveOnChainTransactions,
    RemoveTransactionByHash, RemoveTransactionsByHashes,
};
use crate::services::blockchain::stats::get_stats;
use crate::services::db_config_service::USERS_COL_FAMILY;
use anyhow::Result;
use base::hex_utils::short_hex_string;
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
        // this block execution of transactions that were already processed in a previous block
        mem_pool.call(RemoveOnChainTransactions).await??;

        // get all pending txs from mem-pool
        let transactions_map = mem_pool.call(GetTransactions).await??;

        if transactions_map.is_empty() {
            info!("mem pool empty");
            return Ok(None);
        }

        // get current blockchain stats and tokenomics
        let stats = get_stats().await?;
        let tokenomics = Tokenomics::new(stats.clone());
        let block_height = stats.tip_height + 1;
        let mut tx_hashes: Vec<Vec<u8>> = vec![];

        // the block event for the new block
        let mut block_event = BlockEvent::new(block_height);

        // new signups txs in this block indexed by mobile number - used for referral reward calculations

        let mut sign_ups: HashMap<Vec<u8>, SignedTransaction> = HashMap::new();

        for (tx_hash, tx) in transactions_map.iter() {
            let tx_body = match tx.get_body() {
                Ok(body) => body,
                Err(_) => {
                    continue;
                }
            };

            if tx_body.get_tx_type()? != NewUserV1 {
                // in this loop we only process new user transactions
                continue;
            }

            info!("processing new user tx: {:?}", short_hex_string(tx_hash));

            // the transaction event for the new user transaction
            let mut tx_event = TransactionEvent::new(block_height, tx, tx_hash);

            match self
                .process_new_user_transaction(tx, &tokenomics, &mut tx_event)
                .await
            {
                Ok(res) => {
                    info!(
                        "new user tx hash {} processed: {}",
                        short_hex_string(tx_hash),
                        tx_event
                    );
                    tx_hashes.push(tx_hash.to_vec());
                    block_event.signups_count += 1;
                    block_event.signup_rewards_amount += tx_event.signup_reward;
                    block_event.add_fee(tx_body.fee);
                    block_event.add_transaction_event(tx_event.clone());
                    // update new signups map - used for referrals and payments
                    sign_ups.insert(res.mobile_number.as_bytes().to_vec(), tx.clone());
                }
                Err(e) => {
                    error!(
                        "Failed to process new user tx {}: {:?}",
                        short_hex_string(tx_hash),
                        e
                    );
                    tx_event.result = ExecutionResult::Invalid as i32;
                    tx_event.info = e.execution_info as i32;
                    tx_event.error_message = e.error_message;
                }
            }

            // remove the processed tx from the pool
            mem_pool
                .call(RemoveTransactionByHash(tx_hash.to_vec()))
                .await??;

            // emit the tx event
            self.emit_tx_event(tx_event).await?;
        }

        // process other transactions types (update user and payment
        for (tx_hash, tx) in mem_pool.call(GetTransactions).await??.iter() {
            let tx_body = match tx.get_body() {
                Ok(body) => body,
                Err(_) => {
                    info!("missing tx body");
                    continue;
                }
            };

            let tx_type = tx_body.get_tx_type()?;
            let mut tx_event = TransactionEvent::new(block_height, tx, tx_hash);

            // Get tx issuer user from chain and IGNORE tx if it doesn't exist
            let mut user = match DatabaseService::read(ReadItem {
                key: Bytes::from(tx.signer.as_ref().unwrap().data.clone()),
                cf: USERS_COL_FAMILY,
            })
            .await?
            {
                Some(data) => User::decode(data.0.as_ref())?,
                None => {
                    info!(
                        "Tx signer user not found on chain - ignoring tx and leaving it in mempool"
                    );
                    continue;
                }
            };

            // check nonce and ignore txs with wrong nonce here
            // Nonce checking is disabled until design is figured out
            /*
            if tx_body.validate_nonce(user.nonce).is_err() {
                info!("Tx nonce is invalid - ignoring tx and leaving it in mempool");
                continue;
            }*/

            match tx_type {
                TransactionType::PaymentV1 => {
                    if let Some(mut payee) = BlockChainService::get_payee_user(tx).await? {
                        match self
                            .process_payment_transaction(
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
                                info!("payment transaction processed: {}", tx_event);
                                tx_hashes.push(tx_hash.to_vec());
                                block_event.payments_count += 1;
                                block_event.add_fee(tx_body.fee);
                                if tx_event.referral_reward != 0 {
                                    block_event.referral_rewards_count += 1;
                                    block_event.referral_rewards_amount += tx_event.referral_reward;
                                }
                                if tx_event.appreciation_char_trait_idx != 0 {
                                    block_event.appreciations_count += 1;
                                }

                                block_event.add_transaction_event(tx_event.clone());
                            }
                            Err(e) => {
                                info!("payment transaction failed: {:?}", e);
                                tx_event.result = ExecutionResult::Invalid as i32;
                                tx_event.error_message = e.to_string();
                            }
                        }

                        self.emit_tx_event(tx_event).await?;
                    } else {
                        info!("Payee user not found on chain - keeping this tx in the mem pool for later processing...");
                        continue;
                    }
                }
                TransactionType::UpdateUserV1 => {
                    info!("processing update user transaction");
                    // Get tx signer user from chain and reject tx if it doesn't exist
                    match DatabaseService::read(ReadItem {
                        key: Bytes::from(tx.signer.as_ref().unwrap().data.clone()),
                        cf: USERS_COL_FAMILY,
                    })
                    .await?
                    {
                        Some(data) => {
                            info!("user found on chain");
                            User::decode(data.0.as_ref())?
                        }
                        None => {
                            info!("Tx signer not on chain - rejecting & removing tx from pool");

                            mem_pool
                                .call(RemoveTransactionByHash(tx_hash.to_vec()))
                                .await??;
                            tx_event.result = ExecutionResult::Invalid as i32;
                            tx_event.error_message =
                                "Tx signer user not found on chain - discarding tx from mem pool"
                                    .to_string();

                            self.emit_tx_event(tx_event.clone()).await?;
                            continue;
                        }
                    };

                    match self
                        .process_update_transaction(tx, &tokenomics, &mut tx_event)
                        .await
                    {
                        Ok(_) => {
                            info!("update user transaction processed: {}", tx_event);
                            tx_hashes.push(tx_hash.to_vec());
                            block_event.add_fee(tx_body.fee);
                            block_event.add_transaction_event(tx_event.clone());
                            block_event.user_updates_count += 1;
                        }
                        Err(e) => {
                            error!("Failed to process update user transaction: {:?}", e);
                            tx_event.result = ExecutionResult::Invalid as i32;
                            tx_event.error_message = e.to_string();
                        }
                    }
                    self.emit_tx_event(tx_event).await?;
                }
                _ => {
                    // ignore any other transaction types
                }
            }
        }

        if tx_hashes.is_empty() {
            info!("no txs to add to the block - skip block creation....");
            return Ok(None);
        }

        // create the block
        let block = self
            .create_block(
                &tx_hashes,
                stats,
                &tokenomics,
                block_event,
                block_height,
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
