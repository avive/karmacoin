// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::register_number::RegisterNumber;
use crate::services::verifier::verify_number::Verify;
use anyhow::Result;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::{KeyPair, VerifyNumberResponse};
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierService as VerifierServiceTrait;
use base::karma_coin::karma_coin_verifier::{
    RegisterNumberRequest, RegisterNumberResponse, VerifyNumberRequest,
};
use base::server_config_service::{GetVerifierKeyPair, ServerConfigService};
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct VerifierService {
    key_pair: Option<KeyPair>,
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("Verifier Service created");
        VerifierService { key_pair: None }
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");
        Ok(())
    }
}

impl Service for VerifierService {}

impl VerifierService {
    pub(crate) async fn get_key_pair(&mut self) -> Result<KeyPair> {
        if let Some(key_pair) = &self.key_pair {
            info!("returning cached verifier id key-pair");
            return Ok(key_pair.clone());
        }

        let key_pair: KeyPair = ServerConfigService::from_registry()
            .await?
            .call(GetVerifierKeyPair)
            .await??;

        info!(
            "got key-pair from config service. Verifier account id: {:?}",
            short_hex_string(key_pair.public_key.as_ref().unwrap().key.as_slice())
        );

        self.key_pair = Some(key_pair.clone());
        Ok(key_pair)
    }
}

#[tonic::async_trait]
impl VerifierServiceTrait for VerifierService {
    /// User requests to register a mobile phone number
    async fn register_number(
        &self,
        request: Request<RegisterNumberRequest>,
    ) -> std::result::Result<Response<RegisterNumberResponse>, Status> {
        info!("register number called...");

        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service
            .call(RegisterNumber(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        Ok(Response::new(res))
    }

    /// User requests to verify a number with code received via text message
    async fn verify_number(
        &self,
        request: Request<VerifyNumberRequest>,
    ) -> std::result::Result<Response<VerifyNumberResponse>, Status> {
        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service
            .call(Verify(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        Ok(Response::new(res))
    }
}

// write a unit test for register_number() and verify_number() methods
#[cfg(test)]
mod tests {
    //use super::*;

    #[tokio::test]
    async fn test_register_number() {
        //let service = VerifierService::default();
    }
}
