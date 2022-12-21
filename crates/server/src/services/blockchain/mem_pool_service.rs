// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_core_types::{MemPool, SignedTransaction};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{TXS_POOL_COL_FAMILY, TXS_POOL_KEY};

/// A simple transactions pool service
/// This service is used to store transactions that are not yet included in a block
/// todo: auto prune old transactions from pool
/// todo: add limit to the size of the pool
/// todo: add support for reading pool size
#[derive(Debug, Clone, Default)]
pub(crate) struct MemPoolService {
    pub(crate) transactions : Vec<SignedTransaction>
}

#[async_trait::async_trait]
impl Actor for MemPoolService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {

        // load all transactions from the db
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(TXS_POOL_KEY.as_bytes()),
            cf: TXS_POOL_COL_FAMILY,
        }).await? {
            self.transactions = MemPool::decode(data.0.as_ref())?.transactions

        } else {
            self.transactions = vec![];
        }

        info!("MemPoolService started");

        Ok(())
    }
}

impl Service for MemPoolService {}

#[message(result = "Result<Vec<SignedTransaction>>")]
pub(crate) struct GetTransactions;

/// Create a block with zero or more transactions
#[async_trait::async_trait]
impl Handler<GetTransactions> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetTransactions,
    ) -> Result<Vec<SignedTransaction>> {
        Ok(self.transactions.clone())
    }
}

impl MemPoolService {
    /// Persist the mem_pool to the db
    pub(crate) async fn persist(&self) -> Result<()> {
        let mem_pool = MemPool {
            transactions: self.transactions.clone()
        };

        let mut buf = Vec::with_capacity(mem_pool.encoded_len());
        mem_pool.encode(&mut buf)?;

        // persist the mem_pool
        DatabaseService::write(
            WriteItem {
                data: DataItem {
                    key:  Bytes::from(TXS_POOL_KEY.as_bytes()),
                    value: Bytes::from(buf)
                },
                cf: TXS_POOL_COL_FAMILY,
                ttl: 0,
            }).await
    }
}

#[message(result = "Result<()>")]
pub(crate) struct AddTransaction(pub(crate) SignedTransaction);

/// Create a block with zero or more transactions
#[async_trait::async_trait]
impl Handler<AddTransaction> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: AddTransaction,
    ) -> Result<()> {
        self.transactions.push(msg.0);
        self.persist().await
    }
}





