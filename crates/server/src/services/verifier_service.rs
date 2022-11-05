// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use std::fmt::Error;
use anyhow::{anyhow, Result};
use byteorder::{LittleEndian};
use bytes::Bytes;
use byteorder::{BigEndian, ByteOrder};
use rand_chacha::ChaCha20Rng;

use rand::prelude::*;
use rand_core::SeedableRng;

use base::karma_coin::karma_coin_verifier::phone_numbers_verifier_service_server::PhoneNumbersVerifierService;

use tonic::{Code, IntoRequest, Request, Response, Status};
use base::karma_coin::karma_coin_api::{GetBlockchainEventsRequest, GetBlockchainEventsResponse, GetCharTraitsRequest,
                                       GetCharTraitsResponse, GetNetInfoRequest, GetNetInfoResponse, GetPhoneVerifiersRequest,
                                       GetPhoneVerifiersResponse, GetTransactionRequest, GetTransactionResponse, GetTransactionsRequest,
                                       GetTransactionsResponse, GetUserInfoByAccountRequest, GetUserInfoByAccountResponse,
                                       GetUserInfoByNumberRequest, GetUserInfoByNumberResponse, SubmitTransactionRequest, SubmitTransactionResponse};
use base::karma_coin::karma_coin_core_types::User;
use base::karma_coin::karma_coin_verifier::{RegisterNumberRequest, RegisterNumberResponse, SignUpUserRequest, SignUpUserResponse, SignUpUserResult};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use crate::services::db_config_service::{MOBILE_NUMBERS_COL_FAMILY, VERIFICATION_CODES_COL_FAMILY};

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct VerifierService {}

impl Default for VerifierService {
    fn default() -> Self {
        info!("VerifierService created");
        VerifierService {}
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

/// ApiService implements the ApiServiceTrait trait which defines the grpc rpc methods it provides for clients over the network
#[tonic::async_trait]
impl PhoneNumbersVerifierService for VerifierService {
    async fn register_number(&self, request: Request<RegisterNumberRequest>) -> std::result::Result<Response<RegisterNumberResponse>, Status> {

         let service = VerifierService::from_registry().await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        let res = service.call(RegisterNumber(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    async fn sign_up_user(&self, request: Request<SignUpUserRequest>) -> std::result::Result<Response<SignUpUserResponse>, Status> {
        todo!()
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
        let account_id = req.account_id.ok_or(anyhow!("missing account id"))?;
        let phone_number = req.mobile_number.ok_or(anyhow!("missing mobile phone number"))?;
        let signature = req.signature.ok_or(anyhow!("missing signature"))?;

        // todo: check signature by accountId private key on data so we know caller has private key for accountId

        // check if number is already registered to another user
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(bincode::serialize(&phone_number.number).unwrap()),
            cf: MOBILE_NUMBERS_COL_FAMILY
        }).await? {
            // number already registered for a user account
            let existing_user: User = bincode::deserialize(&user_data.0)?;

            return if existing_user.account_id.unwrap().data == account_id.data {
                Ok(RegisterNumberResponse {
                    result: SignUpUserResult::PhoneAlreadyRegisteredThisAccount as i32
                })
            } else {
                Ok(RegisterNumberResponse {
                    result: SignUpUserResult::PhoneAlreadyRegisteredOtherAccount as i32
                })
            }
        }

        
        // todo: send new verification code via sms to user


        // generate a random 6 digits code
        let code = ChaCha20Rng::from_entropy().gen_range(100_000..999_999);
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, code as u32);

        // store verificationCode -> accountNumber with ttl of 24 hours in

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(buf.to_vec()),
                value: Bytes::from(bincode::serialize(&account_id).unwrap()) },
            cf: VERIFICATION_CODES_COL_FAMILY,
            ttl: 60 * 60 * 24, // 24 hours ttl
        }).await?;


        Ok(RegisterNumberResponse {
            result: SignUpUserResult::CodeSent as i32
        })

    }
}

