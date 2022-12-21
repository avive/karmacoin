// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_core_types::*;
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use db::types::IntDbKey;
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::get_head_height::get_tip;
use crate::services::blockchain::new_user_tx_processor;
use crate::services::db_config_service::{BLOCK_EVENTS_COL_FAMILY, BLOCK_TIP_KEY, BLOCKS_COL_FAMILY, BLOCKCHAIN_DATA_COL_FAMILY};

#[message(result = "Result<Block>")]
pub(crate) struct CreateBlock {
    pub(crate) transactions: Vec<SignedTransaction>,
}

/// Create a block with zero or more transactions
#[async_trait::async_trait]
impl Handler<CreateBlock> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: CreateBlock,
    ) -> Result<Block> {
        if msg.transactions.is_empty() {
            return Err(anyhow::anyhow!("No transactions to create a block"));
        }

        let height = get_tip().await?;

        let mut tx_hashes: Vec<Vec<u8>> = vec![];
        let mut block_events = BlockEvents { events: vec![] };
        let mut total_new_users = 0;
        let mut total_new_users_reward= 0;
        let mut total_new_users_referral_reward= 0;

        let mut total_fees = Amount {
            value: 0,
            coin_type: CoinType::Core as i32
        };

        for tx in msg.transactions.iter() {
            // process each transaction
            match tx.get_tx_type()? {
                TransactionType::NewUserV1 => {
                    match new_user_tx_processor::process_transaction(tx, height + 1).await {
                        Ok(event) => {
                            info!("new user transaction processed: {:?}", event);
                            tx_hashes.push(event.transaction_hash.to_vec());
                            total_new_users += 1;
                            total_fees.value += event.transaction.as_ref().unwrap().fee.as_ref().unwrap().value;
                            block_events.events.push(event);
                        },
                        Err(e) => {
                            error!("Failed to process new user transaction: {:?}", e);
                        }
                    }
                },
                TransactionType::PaymentV1 => {
                    todo!("process payment transaction");
                },
                TransactionType::UpdateUserV1 => {
                    todo!("process update user transaction");
                },
            }
        }

        let block = BlockChainService::create_block_helper(tx_hashes,
                                                     total_fees,
                                                     block_events,
                                                     height + 1,
                                                     self.id_key_pair.as_ref().unwrap()).await?;

        Ok(block)
    }
}

/// BlockchainService block creation implementation
impl BlockChainService {
    /// Create a block with the provided txs hashes at a given height
    /// Internal help method
    async fn create_block_helper(transactions_hashes: Vec<Vec<u8>>,
                           fees: Amount, // tx fees for block producer
                           events: BlockEvents,
                           height: u64,
                           key_pair: &KeyPair
    ) -> Result<Block> {
        let mut block = Block {
            author: None,
            height,
            transactions_hashes,
            fees: Some(fees),
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

        // Persist block events
        let mut buf = Vec::with_capacity(events.encoded_len());
        events.encode(&mut buf)?;
        DatabaseService::write(
            WriteItem {
                data: DataItem {
                    key: IntDbKey::from(height).0,
                    value: Bytes::from(buf)
                },
                cf: BLOCK_EVENTS_COL_FAMILY,
                ttl: 0,
            }).await?;


        // update the chain tip
        let mut tip_buf = [0; 8];
        BigEndian::write_u64(&mut tip_buf, height);
        DatabaseService::write(
            WriteItem {
                data: DataItem {
                    key: BLOCK_TIP_KEY.into(),
                    value: Bytes::from(tip_buf.as_ref().to_vec()) },
                cf: BLOCKCHAIN_DATA_COL_FAMILY,
                ttl: 0,
            }
        ).await?;

        Ok(block)

    }
}



