// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::sms_invites_sender::SendInvites;
use crate::services::verifier::verify_number::Verify;
use crate::services::verifier::verify_number_ex::VerifyEx;
use anyhow::{anyhow, Result};
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_auth::auth_service_client::AuthServiceClient;
use base::karma_coin::karma_coin_core_types::{AccountId, KeyPair};
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierService as VerifierServiceTrait;
use base::karma_coin::karma_coin_verifier::{
    VerifyNumberRequest, VerifyNumberRequestEx, VerifyNumberResponse, VerifyNumberResponseEx,
};
use base::server_config_service::{
    GetVerifierIdKeyPair, ServerConfigService, AUTH_SERVICE_HOST_KEY, AUTH_SERVICE_PORT_KEY,
    AUTH_SERVICE_PROTOCOL_KEY, SEND_INVITE_SMS_MESSAGES_CONFIG_KEY,
    SEND_INVITE_SMS_TASK_FREQ_SECS_CONFIG_KEY,
};
use tokio::spawn;
use tokio_schedule::{every, Job};
use tonic::transport::Channel;
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug)]
pub(crate) struct VerifierService {
    key_pair: Option<KeyPair>,
    pub(crate) auth_client: Option<AuthServiceClient<Channel>>,
    pub(crate) sms_gateway_endpoint: Option<String>,
    pub(crate) sms_gateway_from_number: Option<String>,
    pub(crate) sms_gateway_auth_token: Option<String>,
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("Verifier Service created");
        VerifierService {
            key_pair: None,
            auth_client: None,
            sms_gateway_endpoint: None,
            sms_gateway_from_number: None,
            sms_gateway_auth_token: None,
        }
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");

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

        let send_invites =
            ServerConfigService::get_bool(SEND_INVITE_SMS_MESSAGES_CONFIG_KEY.into())
                .await?
                .unwrap();

        let send_invite_task_period =
            ServerConfigService::get_u64(SEND_INVITE_SMS_TASK_FREQ_SECS_CONFIG_KEY.into())
                .await?
                .unwrap() as u32;

        if send_invites {
            let send_sms_task = every(send_invite_task_period).seconds().perform(|| async {
                let service = VerifierService::from_registry().await;
                if service.is_err() {
                    error!("VerifierService not available");
                    return;
                }
                info!("starting periodic send sms invites task...");
                match service.unwrap().call(SendInvites).await {
                    Ok(res) => {
                        info!("Invites sent task completed");
                        match res {
                            Ok(_) => info!("Invites task completed"),
                            Err(e) => error!("Invites task error: {}", e),
                        }
                    }
                    Err(e) => error!("Error running invites task: {}", e),
                }
            });
            spawn(send_sms_task);
        } else {
            info!("verifier is configured NOT to send sms invites");
        }

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

    async fn verify_number_ex(
        &self,
        request: Request<VerifyNumberRequestEx>,
    ) -> Result<Response<VerifyNumberResponseEx>, Status> {
        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        match service
            .call(VerifyEx(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
        {
            Ok(data) => {
                info!("verification successful");
                Ok(Response::new(data))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }
}
