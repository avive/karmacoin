// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::collections::HashMap;
use anyhow::Result;
use bytes::Bytes;
use prost::Message;
use base::karma_coin::karma_coin_core_types::{MemPool, SignedTransaction};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{TXS_POOL_COL_FAMILY, TXS_POOL_KEY};

/// Max pool size
const MAX_SIZE: usize = 5_000;

/// A simple transactions pool service
/// This service is used to store transactions that are not yet included in a block
#[derive(Debug, Clone, Default)]
pub(crate) struct MemPoolService {
    pub(crate) transactions : HashMap<Vec<u8>, SignedTransaction>
}

#[async_trait::async_trait]
impl Actor for MemPoolService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {

        // load all transactions from the db
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(TXS_POOL_KEY.as_bytes()),
            cf: TXS_POOL_COL_FAMILY,
        }).await? {
            self.transactions = HashMap::new();
            let mem_pool = MemPool::decode(data.0.as_ref())?;
            for tx in mem_pool.transactions {
                self.transactions.insert(tx.get_hash().unwrap().as_ref().to_vec(), tx);
            }
        }

        info!("MemPoolService started. Pool size: {:?}", self.transactions.len());

        Ok(())
    }
}

impl Service for MemPoolService {}

#[message(result = "Result<HashMap<Vec<u8>,SignedTransaction>>")]
pub(crate) struct GetTransactions;

/// Returns the full pool content
#[async_trait::async_trait]
impl Handler<GetTransactions> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetTransactions,
    ) -> Result<HashMap<Vec<u8>,SignedTransaction>> {
        Ok(self.transactions.clone())
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

        if self.transactions.len() > MAX_SIZE {
            return Err(anyhow::anyhow!("Mempool is full - transaction discarded"));
        }

        let tx = msg.0;
        self.transactions.insert(tx.get_hash().unwrap().as_ref().to_vec(), tx);
        self.persist().await
    }
}

#[message(result = "Result<()>")]
pub(crate) struct RemoveTransactionByHash(pub(crate) Vec<u8>);

/// Remove a transaction by a hash
#[async_trait::async_trait]
impl Handler<RemoveTransactionByHash> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RemoveTransactionByHash,
    ) -> Result<()> {
        self.transactions.remove(msg.0.as_slice());
        self.persist().await
    }
}

#[message(result = "Result<()>")]
pub(crate) struct RemoveTransaction(pub(crate) SignedTransaction);

/// Remove a transaction by value
#[async_trait::async_trait]
impl Handler<RemoveTransaction> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RemoveTransaction,
    ) -> Result<()> {
        self.transactions.remove(&*msg.0.get_hash().unwrap().as_ref().to_vec());
        self.persist().await
    }
}

#[message(result = "Result<u64>")]
pub(crate) struct PoolSize;

/// Create a block with zero or more transactions
#[async_trait::async_trait]
impl Handler<PoolSize> for MemPoolService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: PoolSize,
    ) -> Result<u64> {
        Ok(self.transactions.len() as u64)
    }
}

impl MemPoolService {
    /// Persist the mem_pool to the db
    /// internal helper function
    async fn persist(&self) -> Result<()> {
        let mem_pool = MemPool {
            transactions: self.transactions.values().cloned().collect()
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




