// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_blockchain::{CreateBlockRequest, CreateBlockResponse};
use base::karma_coin::karma_coin_core_types::{Block, TransactionType};
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::get_head_height::get_tip;
use crate::services::blockchain::new_user_tx_processor_v1;

#[message(result = "Result<CreateBlockResponse>")]
pub(crate) struct CreateBlock(pub CreateBlockRequest);

/// Create a block with zero or more transactions
#[async_trait::async_trait]
impl Handler<CreateBlock> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: CreateBlock,
    ) -> Result<CreateBlockResponse> {

        let req = msg.0;
        let height = get_tip().await?.height;

        // todo: get previous tip block

        let mut tx_hashes:Vec<Vec<u8>> = vec![];
        for tx in req.transactions.iter() {
            // process each transaction
            match tx.get_tx_type()? {
                TransactionType::NewUserV1 => {
                    match new_user_tx_processor_v1::process_transaction(tx,height+1).await {
                        Ok(event) => {
                            info!("new user transaction processed: {:?}", event);
                            tx_hashes.push(event.transaction_hash.to_vec());
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

        // create block and sign it

        let mut _block = Block {
            author: None,
            height: height+1,
            transactions_hashes: tx_hashes,
            signature: None,
            prev_block_digest: vec![],
            digest: vec![],
        };

        // insert the block to the db

        // update the tip


        todo!()
    }
}


