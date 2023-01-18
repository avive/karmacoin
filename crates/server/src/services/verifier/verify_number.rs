// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::db_config_service::{USERS_COL_FAMILY, USERS_NAMES_COL_FAMILY};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::{anyhow, Result};
use base::karma_coin::karma_coin_core_types::{User, VerifyNumberResponse, VerifyNumberResult::*};
use base::karma_coin::karma_coin_verifier::VerifyNumberRequest;
use base::signed_trait::SignedTrait;
use bytes::Bytes;
use db::db_service::{DatabaseService, ReadItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<VerifyNumberResponse>")]
pub(crate) struct Verify(pub VerifyNumberRequest);

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<Verify> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: Verify,
    ) -> Result<VerifyNumberResponse> {
        let req = msg.0;

        // verify request signature
        req.verify_signature()?;

        let account_id = req
            .account_id
            .ok_or_else(|| anyhow!("missing account id"))?;

        let phone_number = req
            .mobile_number
            .ok_or_else(|| anyhow!("missing mobile phone number"))?;

        let requested_user_name = req.requested_user_name.clone();

        let verifier_key_pair = self.get_key_pair().await?.to_ed2559_keypair();

        // todo: use firebase admin api to get user by phone number and get the account id
        // associated with the phone number we'll fake it for now

        // case 1 - same account id registered for this number - approve the request
        // case 2 - no phone reg on firebase users db
        // case 3 - different account id registered for this number - is user updating his device to a new phone number?
        // check that no other account was created with this mobile number

        // check if there's a user for the accountId
        if let Some(user_data) = DatabaseService::read(ReadItem {
            key: Bytes::from(account_id.data.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?
        {
            // An existing user is asking to update his mobile number
            let user = User::decode(user_data.0.as_ref())?;
            if user.user_name != requested_user_name {
                // don't allow giving evidence on requested user name in case of existing user
                let mut resp = VerifyNumberResponse::from(UserNameTaken);
                resp.signature = Some(resp.sign(&verifier_key_pair)?);
                return Ok(resp);
            }
        } else {
            // verify that the requested username not already registered to another user
            let user_name_key = Bytes::from(requested_user_name.as_bytes().to_vec());
            if (DatabaseService::read(ReadItem {
                key: user_name_key.clone(),
                cf: USERS_NAMES_COL_FAMILY,
            })
            .await?)
                .is_some()
            {
                let mut resp = VerifyNumberResponse::from(UserNameTaken);
                resp.signature = Some(resp.sign(&verifier_key_pair)?);
                return Ok(resp);
            }
        }

        // create signed Response and return it
        let mut resp = VerifyNumberResponse::from(Verified);

        let key_pair = self.get_key_pair().await?;

        // signed attestation details - user account id, nickname and verified mobile number
        resp.account_id = Some(account_id);
        resp.verifier_account_id = Some(self.get_account_id().await?);
        resp.user_name = requested_user_name;
        resp.mobile_number = Some(phone_number);
        resp.signature = Some(resp.sign(&key_pair.to_ed2559_keypair())?);
        Ok(resp)
    }
}
