// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verify_number::Verify;
use anyhow::{anyhow, Result};
use base::hex_utils::{hex_string, short_hex_string};
use base::karma_coin::karma_coin_auth::auth_service_client::AuthServiceClient;
use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair};
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierService as VerifierServiceTrait;
use base::karma_coin::karma_coin_verifier::{VerifyNumberRequest, VerifyNumberResponse};
use base::server_config_service::{
    GetVerifierIdKeyPair, ServerConfigService, AUTH_SERVICE_HOST_KEY, AUTH_SERVICE_PORT_KEY,
    AUTH_SERVICE_PROTOCOL_KEY,
};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug)]
pub(crate) struct VerifierService {
    key_pair: Option<KeyPair>,
    pub(crate) auth_client: Option<AuthServiceClient<Channel>>,
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("Verifier Service created");
        VerifierService {
            key_pair: None,
            auth_client: None,
        }
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");

        let key_pair = KeyPair::new();
        info!(
            "temp key pair public: {}",
            hex_string(key_pair.public_key.as_ref().unwrap().key.as_slice())
        );
        info!(
            "temp key pair private: {}",
            hex_string(key_pair.private_key.as_ref().unwrap().key.as_slice())
        );

        let host = ServerConfigService::get(AUTH_SERVICE_HOST_KEY.into())
            .await?
            .unwrap();

        let port = ServerConfigService::get_u64(AUTH_SERVICE_PORT_KEY.into())
            .await?
            .unwrap() as u32;

        let protocol = ServerConfigService::get(AUTH_SERVICE_PROTOCOL_KEY.into())
            .await?
            .unwrap();

        self.auth_client =
            Some(AuthServiceClient::connect(format!("{}://{}:{}", protocol, host, port)).await?);

        Ok(())
    }
}

impl Service for VerifierService {}

impl VerifierService {
    /// Returns the verifier account id
    pub(crate) async fn get_account_id(&mut self) -> Result<AccountId> {
        let key_pair = self.get_key_pair().await?;
        Ok(AccountId {
            data: key_pair
                .public_key
                .as_ref()
                .ok_or_else(|| anyhow!("No public key"))?
                .key
                .to_vec(),
        })
    }

    /// Returns the verifier id key pair
    pub(crate) async fn get_key_pair(&mut self) -> Result<KeyPair> {
        if let Some(key_pair) = &self.key_pair {
            info!(
                "returning cached verifier id key-pair. account id: {:?}",
                short_hex_string(&key_pair.public_key.as_ref().unwrap().key)
            );
            return Ok(key_pair.clone());
        }

        let key_pair: KeyPair = ServerConfigService::from_registry()
            .await?
            .call(GetVerifierIdKeyPair)
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
    /// User requests to verify a number with code received via text message
    async fn verify_number(
        &self,
        request: Request<VerifyNumberRequest>,
    ) -> std::result::Result<Response<VerifyNumberResponse>, Status> {
        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        match service
            .call(Verify(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
        {
            Ok(data) => {
                info!("verification successful");
                Ok(Response::new(VerifyNumberResponse {
                    user_verification_data: Some(data),
                }))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
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
