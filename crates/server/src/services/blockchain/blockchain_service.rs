// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_core_types::KeyPair;
use base::server_config_service::{
    GetBlockProducerIdKeyPair, GetVerifierIdKeyPair, ServerConfigService,
};
use xactor::*;

/// Blockchain service mocks a blockchain node
/// It provides a GRPC service defined in KarmaCoinBlockchainService
/// It is a lower-level API than the KarmaCoin API - designed to be used internally in the server
#[derive(Debug, Clone, Default)]
pub(crate) struct BlockChainService {
    /// block producer id pair
    pub(crate) id_key_pair: Option<KeyPair>,
    pub(crate) verifier_key_pair: Option<KeyPair>,
}

#[async_trait::async_trait]
impl Actor for BlockChainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Started");

        // block producer id key pair
        self.id_key_pair = Some(
            ServerConfigService::from_registry()
                .await?
                .call(GetBlockProducerIdKeyPair)
                .await??,
        );

        // todo: set block producer unique name

        // verifier key pair for validation
        self.verifier_key_pair = Some(
            ServerConfigService::from_registry()
                .await?
                .call(GetVerifierIdKeyPair)
                .await??,
        );

        // apply patch
        self.apply_patch().await?;

        Ok(())
    }
}

impl Service for BlockChainService {}
