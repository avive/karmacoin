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

impl BlockChainService {
    /// emit a transaction processing event
    pub(crate) async fn emit_tx_event(event: TransactionEvent) -> Result<()> {
        let key = event.transaction_hash.clone();
        let mut transaction_events = TransactionEvents { events: vec![] };

        // load any previous tx events for this transaction from store
        if let Some(data) = DatabaseService::read(ReadItem {
            key: Bytes::from(key.clone()),
            cf: TRANSACTIONS_EVENTS_COL_FAMILY,
        })
        .await?
        {
            let events_data = TransactionEvents::decode(data.0.as_ref())?;
            transaction_events.events = events_data.events;
        }

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
