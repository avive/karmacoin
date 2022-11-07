// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_verifier::phone_numbers_verifier_service_server::PhoneNumbersVerifierService;
use tonic::{Request, Response, Status};
use base::hex_utils::hex_from_string;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, RegisterNumberResponse, VerifyNumberRequest};
use base::karma_coin::karma_coin_core_types::{VerifyNumberResponse, KeyPair, PrivateKey, PublicKey};
use base::server_config_service::{ServerConfigService, VERIFIER_ID_PRIVATE_KEY, VERIFIER_ID_PUBLIC_KEY};
use xactor::*;
use crate::services::register_number::RegisterNumber;
use crate::services::verify_number::Verify;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct VerifierService {
    pub(crate) id_key_pair : Option<KeyPair>
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("VerifierService created");
        VerifierService {
            id_key_pair: None,
        }
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");

        // todo: pull keys from config if they exist, otherwise generate new key pair for signing

        match ServerConfigService::get(VERIFIER_ID_PRIVATE_KEY.into())
            .await? {
            Some(key) => {
                // key is a hex string in config
                let private_key_data = hex_from_string(key).unwrap();

                match ServerConfigService::get(VERIFIER_ID_PUBLIC_KEY.into())
                    .await? {
                    Some(pub_key) => {
                        let pub_key_data = hex_from_string(pub_key).unwrap();
                        self.id_key_pair = Some(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey {
                                key: pub_key_data,
                            })
                        });
                        info!("loaded verifier id key pair from config")
                    },
                    None => {
                        panic!("invalid config: missing verifier id public key");
                    }
                }

            },
            None => {
                // no private key in config - generate new key pair
                self.id_key_pair = Some(KeyPair::new());
                info!("Generated a new random verifier id key pair");
            }
        }

        Ok(())
    }
}

impl Service for VerifierService {}

#[tonic::async_trait]
impl PhoneNumbersVerifierService for VerifierService {

    // User requests to register a mobile phone number
    async fn register_number(&self, request: Request<RegisterNumberRequest>) -> std::result::Result<Response<RegisterNumberResponse>, Status> {

         let service = VerifierService::from_registry().await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service.call(RegisterNumber(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    async fn verify_number(&self, request: Request<VerifyNumberRequest>) -> std::result::Result<Response<VerifyNumberResponse>, Status> {

        let service = VerifierService::from_registry().await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service.call(Verify(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }
}



