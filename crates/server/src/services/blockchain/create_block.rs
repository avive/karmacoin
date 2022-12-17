// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_blockchain::{CreateBlockRequest, CreateBlockResponse};
use base::karma_coin::karma_coin_core_types::Block;
use xactor::*;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::get_head_height::get_tip;

#[message(result = "Result<CreateBlockResponse>")]
pub(crate) struct CreateBlock(pub CreateBlockRequest);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<CreateBlock> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: CreateBlock,
    ) -> Result<CreateBlockResponse> {

        let req = msg.0;
        let height = get_tip().await?.height;

        // get last block

        for _tx in req.transactions.iter() {
            // todo: process each transaction

        }

        // create block and sign it

        let mut _block = Block {
            author: None,
            height: height + 1,
            transactions_hashes: vec![],
            signature: None,
            prev_block_digest: vec![],
            digest: vec![],
        };



        // insert the block to the db

        // update the tip


        todo!()
    }
}


