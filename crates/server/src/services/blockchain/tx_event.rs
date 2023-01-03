// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::TRANSACTIONS_EVENTS_COL_FAMILY;
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<TransactionEvents>")]
pub(crate) struct GetTransactionEvents {
    pub(crate) tx_hash: Bytes,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetTransactionEvents> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionEvents,
    ) -> Result<TransactionEvents> {
        self.get_tx_events(msg.tx_hash).await
    }
}

impl BlockChainService {
    /// Get all translation events for a given transaction hash
    pub(crate) async fn get_tx_events(&self, tx_hash: Bytes) -> Result<TransactionEvents> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: tx_hash,
            cf: TRANSACTIONS_EVENTS_COL_FAMILY,
        })
        .await?
        {
            Ok(TransactionEvents::decode(data.0.as_ref())?)
        } else {
            Ok(TransactionEvents::default())
        }
    }
    /// emit a transaction processing event
    pub(crate) async fn emit_tx_event(&self, event: TransactionEvent) -> Result<()> {
        let key = event.transaction_hash.clone();
        let mut transaction_events = self.get_tx_events(Bytes::from(key.clone())).await?;

        transaction_events.events.push(event.clone());
        let mut buf = Vec::with_capacity(transaction_events.encoded_len());
        transaction_events.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(key),
                value: Bytes::from(buf),
            },
            cf: TRANSACTIONS_EVENTS_COL_FAMILY,
            ttl: 0, // todo: ttl should be based on node being archive or standard....
        })
        .await?;

        info!("Tx event emitted: {}", event);

        Ok(())
    }
}
