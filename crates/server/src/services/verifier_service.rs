// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::convert::TryFrom;
use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ByteOrder};
use bytes::Bytes;
use ed25519_dalek::{Keypair , Verifier};
use rand_chacha::ChaCha20Rng;
use rand::prelude::*;
use rand_core::SeedableRng;
use base::karma_coin::karma_coin_verifier::phone_numbers_verifier_service_server::PhoneNumbersVerifierService;
use tonic::{Request, Response, Status};
use base::hex_utils::hex_from_string;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, RegisterNumberResponse, RegisterNumberResult, VerifyNumberRequest};
use base::karma_coin::karma_coin_core_types::{VerifyNumberResult, VerifyNumberResponse, VerifyNumberResult::*, KeyPair, PrivateKey, PublicKey};
use base::server_config_service::{ServerConfigService, VERIFIER_ID_PRIVATE_KEY, VERIFIER_ID_PUBLIC_KEY};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, VERIFICATION_CODES_COL_FAMILY};

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct VerifierService {
    id_key_pair : Option<KeyPair>
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

#[message(result = "Result<VerifyNumberResponse>")]
pub(crate) struct Verify(VerifyNumberRequest);

// Request to sign up
#[async_trait::async_trait]
impl Handler<Verify> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: Verify,
    ) -> Result<VerifyNumberResponse> {

        let req = msg.0;

        // Verify signature
        let mut cloned_req = req.clone();
        cloned_req.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(cloned_req.encoded_len());
        if cloned_req.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let account_id = req.account_id.ok_or(anyhow!("missing account id"))?;
        let signature_data = req.signature.ok_or(anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature)?;

        // decode auth code number
        let auth_code : u32 = req.code.parse::<u32>().map_err(|_| anyhow!("invalid auth code"))?;

        // db key based on auth code
        let mut auth_code_buf = [0; 4];
        LittleEndian::write_u32(&mut auth_code_buf, auth_code);

        let auth_data = DatabaseService::read(ReadItem {
            key: Bytes::from(auth_code_buf.to_vec()),
            cf: VERIFICATION_CODES_COL_FAMILY
        }).await?;

        if auth_data.is_none() {
            return Ok(VerifyNumberResponse::from(InvalidCode));
        }

        // check that code was sent to the caller's account id
        let sent_account_id = auth_data.unwrap().0.to_vec();
        if account_id.data != sent_account_id {
            // code was sent to a different account
            return Ok(VerifyNumberResponse::from(InvalidCode));
        }

        // todo: check that no other account was created with this mobile number

        let phone_number = req.mobile_number.ok_or(anyhow!("missing mobile phone number"))?;

        if let Some(_) = DatabaseService::read(ReadItem {
            key: Bytes::from(bincode::serialize(&phone_number.number).unwrap()),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await? {
            return Ok(VerifyNumberResponse::from(NumberAlreadyRegisteredOtherAccount));
        }

        // check for unique nickname requested

        let nick_name_key = bincode::serialize(&req.nickname).unwrap();
        if let Some(_) = DatabaseService::read(ReadItem {
            key: Bytes::from(nick_name_key),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await? {
            return Ok(VerifyNumberResponse::from(NicknameTaken));
        }

        // todo: create signed Response and return it

        Ok(VerifyNumberResponse::from(Verified))

    }
}


#[message(result = "Result<RegisterNumberResponse>")]
pub(crate) struct RegisterNumber(RegisterNumberRequest);

// Request to register a phone number for an account
#[async_trait::async_trait]
impl Handler<RegisterNumber> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: RegisterNumber,
    ) -> Result<RegisterNumberResponse> {

        let req = msg.0;

        // Verify signature
        let mut cloned_req = req.clone();
        cloned_req.signature = None;
        use prost::Message;
        let mut buf = Vec::with_capacity(cloned_req.encoded_len());
        if cloned_req.encode(&mut buf).is_err() {
            return Err(anyhow!("failed to encode source data to binary data"));
        };

        let account_id = req.account_id.ok_or(anyhow!("missing account id"))?;
        let signature_data = req.signature.ok_or(anyhow!("missing signature"))?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature_data.signature)?;
        let signer_pub_key = ed25519_dalek::PublicKey::from_bytes(account_id.data.as_slice())?;
        signer_pub_key.verify(&buf, &signature)?;

        let phone_number = req.mobile_number.ok_or(anyhow!("missing mobile phone number"))?;

        // check signature by accountId private key on data so we know caller has private key for accountId


        // check if number is already registered to another user
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(bincode::serialize(&phone_number.number).unwrap()),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await? {
            // number already registered for a user account

            // compare account ids
            return if user_data.0 == account_id.data {
                Ok(RegisterNumberResponse {
                    result: VerifyNumberResult::NumberAlreadyRegisteredThisAccount as i32
                })
            } else {
                Ok(RegisterNumberResponse {
                    result: VerifyNumberResult::NumberAlreadyRegisteredOtherAccount as i32
                })
            }
        }

        // todo: send new verification code via sms to user

        // generate a random 6 digits code
        let code = ChaCha20Rng::from_entropy().gen_range(100_000..999_999);
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, code as u32);

        info!("Sent verification code {:?}, to accountID: {:?}", code, account_id.data);

        // store verificationCode -> accountNumber with ttl of 24 hours in

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(buf.to_vec()),
                value: Bytes::from(account_id.data.to_vec()) },
            cf: VERIFICATION_CODES_COL_FAMILY,
            ttl: 60 * 60 * 24, // 24 hours ttl
        }).await?;


        Ok(RegisterNumberResponse {
            result: RegisterNumberResult::CodeSent as i32
        })

    }
}

