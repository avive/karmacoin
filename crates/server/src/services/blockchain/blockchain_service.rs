// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_core_types::KeyPair;
use base::server_config_service::{GetBlockProducerIdKeyPair, ServerConfigService};
use xactor::*;


/// Blockchain service mocks a blockchain node
/// It provides a GRPC service defined in KarmaCoinBlockchainService
/// It is a lower-level API than the KarmaCoin API - designed to be used internally in the server
#[derive(Debug, Clone, Default)]
pub(crate) struct BlockChainService {
    pub(crate) id_key_pair : Option<KeyPair>
}

#[async_trait::async_trait]
impl Actor for BlockChainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("BlockChainService started");

        self.id_key_pair  = Some(ServerConfigService::from_registry().await?.call(GetBlockProducerIdKeyPair).await??);
        Ok(())
    }
}

impl Service for BlockChainService {}





