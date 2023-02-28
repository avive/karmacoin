use crate::services::blockchain::mem_pool_service::{GetTransactions, MemPoolService};
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::anyhow;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::{
    SignedTransaction, TransactionBody, TransactionType, User,
};
use base::karma_coin::karma_coin_verifier::SmsInviteMetadata;
use base::server_config_service::{
    ServerConfigService, MAX_SMS_INVITES_PER_NUMBER_CONFIG_KEY,
    SEND_INVITE_SMS_TIME_BETWEEN_SMS_SECS_CONFIG_KEY,
};
// use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use chrono::Duration;
use chrono::Utc;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use http::StatusCode;
use prost::Message;
// use tonic::transport::Server;

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{INVITE_SMS_COL_FAMILY, USERS_COL_FAMILY};
use xactor::*;

#[message(result = "Result<()>")]
pub(crate) struct SendInvites;

/// Send invites to people based on payment transactions pending in the tx pool
#[async_trait::async_trait]
impl Handler<SendInvites> for VerifierService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: SendInvites) -> Result<()> {
        info!("Sending sms invites...");

        let mem_pool = MemPoolService::from_registry().await?;

        // get all pending txs from mem-pool
        let txs = mem_pool.call(GetTransactions).await??;
        if txs.is_empty() {
            info!("mem pool empty");
            return Ok(());
        }

        let max_invites_per_number =
            ServerConfigService::get_u64(MAX_SMS_INVITES_PER_NUMBER_CONFIG_KEY.into())
                .await?
                .unwrap();

        let cool_down_period =
            ServerConfigService::get_u64(SEND_INVITE_SMS_TIME_BETWEEN_SMS_SECS_CONFIG_KEY.into())
                .await?
                .unwrap();

        if self.sms_gateway_auth_token.is_none() {
            self.sms_gateway_auth_token = Some(
                ServerConfigService::get("verifier.sms_gateway.auth_value".into())
                    .await?
                    .unwrap(),
            );
        }

        if self.sms_gateway_endpoint.is_none() {
            self.sms_gateway_endpoint = Some(
                ServerConfigService::get("verifier.sms_gateway.api_endpoint".into())
                    .await?
                    .unwrap(),
            );
        }

        if self.sms_gateway_from_number.is_none() {
            self.sms_gateway_from_number = Some(
                ServerConfigService::get("verifier.sms_gateway.from_number".into())
                    .await?
                    .unwrap(),
            );
        }

        // shared http client for this iteration
        let client = reqwest::Client::new();

        for (tx_hash, tx) in txs.iter() {
            info!("processing tx: {}", short_hex_string(tx_hash.as_slice()));

            let inviter = match DatabaseService::read(ReadItem {
                key: Bytes::from(tx.signer.as_ref().unwrap().data.clone()),
                cf: USERS_COL_FAMILY,
            })
            .await?
            {
                Some(data) => User::decode(data.0.as_ref())?,
                None => {
                    info!(
                        "Inviter (tx signer) account not found on chain yet - ignoring tx and leaving it in mempool"
                    );
                    continue;
                }
            };

            let tx_body = match tx.get_body() {
                Ok(tx_body) => tx_body,
                Err(_) => {
                    info!("failed to get tx body... skipping tx");
                    continue;
                }
            };

            let tx_type = match tx_body.get_tx_type() {
                Ok(tx_type) => tx_type,
                Err(_) => {
                    info!("failed to get tx type... skipping tx");
                    continue;
                }
            };

            if tx_type != TransactionType::PaymentV1 {
                info!("skipping a non-payment tx");
                continue;
            }

            if (BlockChainService::get_payee_user(tx).await?).is_some() {
                info!("tx payee already has an on-chain account - skipping");
            }

            // payment transaction to a non-user - send him an invite!
            let _ = self
                .send_invite(
                    tx,
                    &tx_body,
                    max_invites_per_number,
                    cool_down_period,
                    &client,
                    &inviter,
                )
                .await
                .map_err(|e| info!("failed to send invite: {}", e));
        }

        Ok(())
    }
}

impl VerifierService {
    async fn send_invite(
        &self,
        signed_tx: &SignedTransaction,
        tx_body: &TransactionBody,
        max_invites_per_number: u64,
        cool_down_period: u64,
        client: &reqwest::Client,
        inviter: &User,
    ) -> Result<()> {
        let payment_tx = tx_body.get_payment_transaction_v1().unwrap();
        let invite_tx_hash = signed_tx.get_hash()?;

        let invite_mobile_number = payment_tx
            .to_number
            .as_ref()
            .ok_or_else(|| anyhow!("missing tx receiver's mobile number"))?;

        info!("Computing sms invite to: {}", invite_mobile_number.number);

        let invite_db_key = Bytes::from(invite_mobile_number.number.as_bytes().to_vec());

        let mut sms_invite_data = match DatabaseService::read(ReadItem {
            key: invite_db_key.clone(),
            cf: INVITE_SMS_COL_FAMILY,
        })
        .await?
        {
            Some(data) => SmsInviteMetadata::decode(data.0.as_ref())?,
            None => SmsInviteMetadata::new(
                invite_mobile_number,
                signed_tx.signer.as_ref().unwrap(),
                invite_tx_hash.as_ref(),
            ),
        };

        let inviter_phone_number = inviter
            .mobile_number
            .as_ref()
            .ok_or_else(|| anyhow!("inviter mobile number not found on chain"))?
            .number
            .clone();

        let now = Utc::now().timestamp_millis() as u64;

        if i64::abs(now as i64 - sms_invite_data.last_message_sent_time_stamp as i64)
            < Duration::seconds(cool_down_period as i64).num_milliseconds()
        {
            info!("last sms sent not too long ago (within cool-down period) - skipping invite");
            return Ok(());
        }

        if sms_invite_data.messages_sent >= max_invites_per_number as u32 {
            info!("already sent the max number of invites to this number - skipping invite");
            return Ok(());
        }

        // todo: get amount of payment tx (non zero) and format it properly in KC and in USD units.
        // people want to know how much coins they got

        // todo: move this out of the loop as these don't change per server instance

        let sms_body = format!(
            "👋 I just appreciated you and sent you some Karma Coins! 🙏
- {} ({})

☯️ To get these, sign up with your mobile number to the Karma Coin App: https://karmaco.in",
            inviter.user_name.clone(),
            inviter_phone_number
        );

        // todo: take from number from config file
        let params = [
            ("To", invite_mobile_number.number.as_str()),
            ("Body", sms_body.as_str()),
            (
                "From",
                self.sms_gateway_from_number.as_ref().unwrap().as_str(),
            ),
        ];

        info!("calling sms gateway api...");

        // todo: get api endpoint url from config file
        match client
            .post(self.sms_gateway_endpoint.as_ref().unwrap().clone())
            .form(&params)
            .header(
                "Authorization",
                self.sms_gateway_auth_token.as_ref().unwrap().clone(),
            )
            .send()
            .await?
            .status()
        {
            StatusCode::CREATED => info!("sms sent via gateway :-)"),
            status => {
                return Err(anyhow!(format!("sms gateway api call failed: {}", status)));
            }
        }

        // Update data and store in the db
        sms_invite_data.last_message_sent_time_stamp = now;
        sms_invite_data.messages_sent += 1;

        let mut buf = Vec::with_capacity(sms_invite_data.encoded_len());
        sms_invite_data.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: invite_db_key,
                value: Bytes::from(buf),
            },
            cf: INVITE_SMS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        Ok(())
    }
}
