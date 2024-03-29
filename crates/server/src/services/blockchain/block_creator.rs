// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    BLOCKS_COL_FAMILY, USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY,
};
use anyhow::{anyhow, Result};
use base::genesis_config_service::ONE_KC_IN_KCENTS;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::*;
use base::server_config_service::{ServerConfigService, BLOCK_PRODUCER_USER_NAME};
use base::signed_trait::SignedTrait;
use bytes::Bytes;
use chrono::Utc;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use db::types::IntDbKey;
use prost::Message;

/// BlockchainService block creation implementation
impl BlockChainService {
    /// apply one-time patch on startup
    pub(crate) async fn _apply_patch(&self) -> Result<()> {
        info!("applying patch...");
        // get block producer user
        let mut block_producer: User = self
            .get_block_producer_user_account(self.id_key_pair.as_ref().unwrap())
            .await?;

        // spending account id (newdeal, +972549805384
        let spending_account_id = vec![
            27, 43, 232, 126, 102, 10, 33, 116, 152, 152, 131, 189, 236, 71, 27, 84, 58, 199, 170,
            204, 80, 179, 111, 53, 25, 101, 50, 89, 140, 26, 197, 35,
        ];

        let mut spending_user = match DatabaseService::read(ReadItem {
            key: Bytes::from(spending_account_id.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            Some(data) => User::decode(data.0.as_ref())?,
            None => {
                return Err(anyhow!("spending account not found"));
            }
        };

        let amount = 10_000 * ONE_KC_IN_KCENTS;

        info!("Transferring {} to {}...", amount, spending_user.user_name);

        if block_producer.balance < amount {
            return Err(anyhow!("insufficient producer balance"));
        }

        // update balances

        spending_user.balance += amount;
        block_producer.balance -= amount;

        // store users in db

        let mut buf = Vec::with_capacity(spending_user.encoded_len());
        spending_user.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(spending_account_id),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        let mut buf = Vec::with_capacity(block_producer.encoded_len());
        block_producer.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(block_producer.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        info!("Transferred {} to {}...", amount, spending_user.user_name);

        Ok(())
    }

    /// Returns this block producer on-chain user account.
    /// Attempts to create one if it doesn't exist using config data (account id and nickname)
    async fn get_block_producer_user_account(&self, key_pair: &KeyPair) -> Result<User> {
        // Get User from chain and reject tx if user doesn't exist
        let block_producer = match DatabaseService::read(ReadItem {
            key: Bytes::from(key_pair.public_key.as_ref().unwrap().key.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            Some(data) => User::decode(data.0.as_ref())?,
            None => {
                // create block producer on-chain account

                let user_name = ServerConfigService::get(BLOCK_PRODUCER_USER_NAME.into())
                    .await?
                    .unwrap();
                let account_id = key_pair.public_key.as_ref().unwrap().key.clone();
                // verify the the requested user-name does not belong to a new user over
                // todo: figure out block producers name stuff - maybe just use account id?
                if (DatabaseService::read(ReadItem {
                    key: Bytes::from(user_name.as_bytes().to_vec()),
                    cf: USERS_NAMES_COL_FAMILY,
                })
                .await?)
                    .is_some()
                {
                    return Err(anyhow::anyhow!(
                        "Nickname {} belongs to another user",
                        user_name
                    ));
                }

                info!(
                    "created block producer account with account id {}",
                    short_hex_string(account_id.as_ref())
                );

                User {
                    account_id: Some(AccountId {
                        data: account_id.clone(),
                    }),
                    nonce: 0,
                    user_name,
                    mobile_number: None, // block producer account starts w/o a verified mobile number
                    balance: 0,
                    trait_scores: vec![],
                    pre_keys: vec![],
                    karma_score: 1,
                    community_memberships: vec![],
                }
            }
        };

        Ok(block_producer)
    }

    /// Create a block with the provided txs hashes at a given height
    /// Internal help method
    pub(crate) async fn create_block(
        &self,
        transactions_hashes: &[Vec<u8>],
        stats: BlockchainStats,
        tokenomics: &Tokenomics,
        mut block_event: BlockEvent,
        height: u64,
        key_pair: &KeyPair,
    ) -> Result<Block> {
        let mut block_producer = self.get_block_producer_user_account(key_pair).await?;

        let mut block = Block {
            time: Utc::now().timestamp_millis() as u64,
            author: Some(block_producer.account_id.as_ref().unwrap().clone()),
            height,
            transactions_hashes: transactions_hashes.to_vec(),
            fees: block_event.fees_amount,
            reward: 0,
            minted: 0,
            signature: None,
            prev_block_digest: vec![],
            digest: vec![],
        };

        // Set previous block hash to the hash of the previous block unless genesis block
        if height != 1 {
            let Some(prev_block_data) = DatabaseService::read(ReadItem {
                key: IntDbKey::from(height -1).0,
                cf: BLOCKS_COL_FAMILY
            }).await? else {
                return Err(anyhow::anyhow!("Failed to read previous block"));
            };

            let prev_block = Block::decode(prev_block_data.0)?;
            block.prev_block_digest = prev_block.digest;
        } else {
            info!("creating genesis block");
        };

        // set block reward
        block.reward = tokenomics.get_block_reward_amount(height).await?;
        info!("block reward: {}", block.reward);

        // sign the block
        block.signature = Some(block.sign(&key_pair.to_ed2559_keypair())?);

        // compute block hash (including the signature) and set it
        block.digest = block.get_hash()?.to_vec();

        // insert the block to the db
        let mut buf = Vec::with_capacity(block.encoded_len());
        info!("binary block size: {}", block.encoded_len());
        block.encode(&mut buf)?;

        // Write the block to the db
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: IntDbKey::from(height).0,
                value: Bytes::from(buf),
            },
            cf: BLOCKS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // Update and persist block event
        block_event.block_hash = block.digest.clone();
        block_event.reward = block.reward;
        self.emit_block_event(&block_event).await?;

        // Update block producer balance with block reward and with fees and persist

        block_producer.balance += block_event.fees_amount + block.reward;

        let mut buf = Vec::with_capacity(block_producer.encoded_len());
        block_producer.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(block_producer.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // Update blockchain global stats and persist
        self.update_blockchain_stats(stats, &block_event, &block)
            .await?;

        Ok(block)
    }
}
