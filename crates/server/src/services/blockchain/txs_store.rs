// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{
    TRANSACTIONS_COL_FAMILY, TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
};
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<(Vec<SignedTransactionWithStatus>, TransactionEvents)>")]
pub(crate) struct GetTransactionsAndEventsByAccountId {
    pub(crate) account_id: Bytes,
}

#[async_trait::async_trait]
impl Handler<GetTransactionsAndEventsByAccountId> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionsAndEventsByAccountId,
    ) -> Result<(Vec<SignedTransactionWithStatus>, TransactionEvents)> {
        self.get_transactions_and_events_by_account_id(msg.account_id)
            .await
    }
}

#[message(result = "Result<Vec<SignedTransactionWithStatus>>")]
pub(crate) struct GetTransactionsByAccountId {
    pub(crate) account_id: Bytes,
}

#[async_trait::async_trait]
impl Handler<GetTransactionsByAccountId> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionsByAccountId,
    ) -> Result<Vec<SignedTransactionWithStatus>> {
        self.get_transactions_by_account_id(msg.account_id).await
    }
}

#[message(result = "Result<Option<SignedTransactionWithStatus>>")]
pub(crate) struct GetTransactionByHash {
    pub(crate) hash: Bytes,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetTransactionByHash> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionByHash,
    ) -> Result<Option<SignedTransactionWithStatus>> {
        self.get_transaction_by_hash(msg.hash).await
    }
}

impl BlockChainService {
    pub(crate) async fn get_transaction_by_hash(
        &self,
        hash: Bytes,
    ) -> Result<Option<SignedTransactionWithStatus>> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: hash,
            cf: TRANSACTIONS_COL_FAMILY,
        })
        .await?
        {
            let tx = SignedTransaction::decode(data.0.as_ref())?;
            Ok(Some(SignedTransactionWithStatus {
                transaction: Some(tx),
                status: OnChain as i32,
            }))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn get_transactions_and_events_by_account_id(
        &self,
        account_id: Bytes,
    ) -> Result<(Vec<SignedTransactionWithStatus>, TransactionEvents)> {
        let txs = self.get_transactions_by_account_id(account_id).await?;
        let mut tx_events = TransactionEvents { events: vec![] };
        for tx_data in txs.iter() {
            let tx = tx_data
                .transaction
                .as_ref()
                .ok_or_else(|| anyhow!("missing transaction in transaction with status"))?;

            let tx_hash = tx.get_hash()?;
            let events_data = self.get_tx_events(tx_hash).await?;
            for event in events_data.events {
                tx_events.events.push(event);
            }
        }

        Ok((txs, tx_events))
    }

    /// Get transactions by account id
    /// These include all transactions to and from this account
    pub(crate) async fn get_transactions_by_account_id(
        &self,
        account_id: Bytes,
    ) -> Result<Vec<SignedTransactionWithStatus>> {
        return if let Some(data) = DatabaseService::read(ReadItem {
            key: account_id,
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
        })
        .await?
        {
            let data = SignedTransactionsHashes::decode(data.0.as_ref())?;
            let mut txs = vec![];

            for tx_hash in data.hashes {
                if let Some(data) = DatabaseService::read(ReadItem {
                    key: Bytes::from(tx_hash),
                    cf: TRANSACTIONS_COL_FAMILY,
                })
                .await?
                {
                    let tx = SignedTransaction::decode(data.0.as_ref())?;
                    txs.push(SignedTransactionWithStatus {
                        transaction: Some(tx),
                        status: OnChain as i32,
                    });
                }
            }
            Ok(txs)
        } else {
            Ok(vec![])
        };
    }

    /// Index a transaction by an account id
    pub(crate) async fn index_transaction_by_account_id(
        &mut self,
        transaction: &SignedTransaction,
        account_id: Bytes,
    ) -> Result<()> {
        let tx_hash = transaction.get_hash()?;

        let tx_hashes = if let Some(data) = DatabaseService::read(ReadItem {
            key: account_id.clone(),
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
        })
        .await?
        {
            let mut data = SignedTransactionsHashes::decode(data.0.as_ref())?;
            data.hashes.push(tx_hash.clone().to_vec());
            data
        } else {
            SignedTransactionsHashes {
                hashes: vec![tx_hash.clone().to_vec()],
            }
        };

        let mut buf = Vec::with_capacity(tx_hashes.encoded_len());
        tx_hashes.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: account_id,
                value: Bytes::from(buf),
            },
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
            ttl: 0,
        })
        .await
    }
}
