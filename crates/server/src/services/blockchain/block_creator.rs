// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use prost::Message;
use base::karma_coin::karma_coin_core_types::*;
use base::karma_coin::karma_coin_core_types::CoinType::Core;
use base::server_config_service::{BLOCK_PRODUCER_USER_NAME, ServerConfigService};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use db::types::IntDbKey;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::stats::write_stats;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{BLOCK_EVENTS_COL_FAMILY, BLOCKS_COL_FAMILY, RESERVED_NICKS_COL_FAMILY, USERS_COL_FAMILY};

/// BlockchainService block creation implementation
impl BlockChainService {

    // Returns this block producer on-chain user account.
    // Attempts to create one if it doesn't exist using config data (account id and nickname)
    async fn get_block_producer_user_account(key_pair: &KeyPair) -> Result<User> {

        // Get User from chain and reject tx if user doesn't exist
        let block_producer = match DatabaseService::read(ReadItem {
            key: Bytes::from(key_pair.public_key.as_ref().unwrap().key.clone()),
            cf: USERS_COL_FAMILY
        }).await? {
            Some(data) => {
                User::decode(data.0.as_ref())?
            },
            None => {
                // create block producer on-chain account

                let user_name = ServerConfigService::get(BLOCK_PRODUCER_USER_NAME.into()).await?.unwrap();
                let account_id = key_pair.public_key.as_ref().unwrap().key.clone();
                // verify the the requested nickname does not belong to a new user over
                if (DatabaseService::read(ReadItem {
                    key: Bytes::from(user_name.as_bytes().to_vec()),
                    cf: RESERVED_NICKS_COL_FAMILY
                }).await?).is_some() {
                    return Err(anyhow::anyhow!("Nickname {} belongs to another user", user_name));
                }

                // update nickname index
                DatabaseService::write(WriteItem {
                    data: DataItem { key: Bytes::from(user_name.as_bytes().to_vec()), value: Bytes::from(account_id.clone()) },
                    cf: RESERVED_NICKS_COL_FAMILY,
                    ttl: 0,
                }).await?;

                User {
                    account_id: Some(AccountId {
                        data: account_id.clone(),
                    }),
                    nonce: 0,
                    user_name,
                    mobile_number: None, // block producer account starts w/o a verified mobile number
                    balances: vec![],
                    trait_scores: vec![],
                    pre_keys: vec![],
                }
            }
        };

        Ok(block_producer)

    }

    /// Create a block with the provided txs hashes at a given height
    /// Internal help method
    pub(crate) async fn create_block(transactions_hashes: &[Vec<u8>],
                                     stats: BlockchainStats,
                                     tokenomics: &Tokenomics,
                                     mut block_event: BlockEvent,
                                     height: u64,
                                     key_pair: &KeyPair
    ) -> Result<Block> {


        let mut block_producer = Self::get_block_producer_user_account(key_pair).await?;

        let mut block = Block {
            time: Utc::now().timestamp_millis() as u64,
            author: Some(block_producer.account_id.as_ref().unwrap().clone()),
            height,
            transactions_hashes: transactions_hashes.to_vec(),
            fees: Some(block_event.fees_amount.as_ref().unwrap().clone()),
            reward: None,
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

        // set block reward

        block.reward = Some(Amount {
            value: tokenomics.get_block_reward_amount(height).await?,
            coin_type: Core as i32,
        });

        // sign the block
        block.sign(&key_pair.to_ed2559_kaypair())?;

        // compute block hash (including the signature) and set it
        block.digest = block.get_hash()?.to_vec();

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

        // Update block producer balance with block reward and with fees and persist

        let mut balance = block_producer.get_balance(Core);
        balance.value += block_event.fees_amount.as_ref().unwrap().value;
        block_producer.update_balance(&balance);
        let mut buf = Vec::with_capacity(block_producer.encoded_len());
        block_producer.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(block_producer.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        }).await?;

        // Update blockchain global stats and persist
        BlockChainService::update_blockchain_stats(stats, &block_event, &block).await?;

        Ok(block)
    }

    /// Update blockchain stats with new block data and store in db
    async fn update_blockchain_stats(mut stats: BlockchainStats, block_event: &BlockEvent, block: &Block) -> Result<()> {

        stats.tip_height += 1;
        stats.users_count += block_event.signups_count;
        stats.fees_amount += block_event.fees_amount.as_ref().unwrap().value;
        stats.transactions_count += block.transactions_hashes.len() as u64;
        stats.last_block_time = block.time;
        stats.payments_transactions_count +=  block_event.payments_count;
        //stats.pa += block_event.payments_count.as_ref().unwrap().value;
        stats.signup_rewards_amount += block_event.signup_rewards_amount.as_ref().unwrap().value;
        stats.signup_rewards_count += block_event.signups_count;

        stats.referral_rewards_amount += block_event.referral_rewards_amount.as_ref().unwrap().value;
        stats.referral_rewards_count += block_event.referral_rewards_count;

        // todo: update tokenomics data

        let mut sub_count = 0;

        for tx_event in block_event.transactions_events.iter() {
            // todo: update subsidies count stats based on the the tx events in the block events (fee type)

        }


        write_stats(stats).await


    }
}

