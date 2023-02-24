use crate::services::blockchain::mem_pool_service::{GetTransactions, MemPoolService};
use crate::services::blockchain::payment_tx_processor::get_payee_user;
use crate::services::verifier::verifier_service::VerifierService;
use anyhow::anyhow;
use base::hex_utils::short_hex_string;
use base::karma_coin::karma_coin_core_types::{
    SignedTransaction, TransactionBody, TransactionType,
};
use base::karma_coin::karma_coin_verifier::SmsInviteMetadata;
use base::server_config_service::{
    ServerConfigService, MAX_SMS_INVITES_PER_NUMBER_CONFIG_KEY, SEND_INVITE_SMS_MESSAGES_CONFIG_KEY,
};
use bytes::Bytes;
use chrono::{Duration, Utc};
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
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

        for (tx_hash, tx) in txs.iter() {
            info!("processing tx: {}", short_hex_string(tx_hash.as_slice()));

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

            if (get_payee_user(tx).await?).is_some() {
                info!("tx payee already has an on-chain account - skipping");
            }

            // payment transaction to a non-user - send him an invite!
            let _ = self.send_invite(tx, &tx_body, max_invites_per_number).await;
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
    ) -> Result<()> {
        let payment_tx = tx_body.get_payment_transaction_v1().unwrap();
        let invite_tx_hash = signed_tx.get_hash()?;

        let invite_mobile_number = payment_tx
            .to
            .as_ref()
            .ok_or_else(|| anyhow!("missing receiver mobile number"))?;

        let invite_db_key = Bytes::from(invite_mobile_number.number.as_bytes().to_vec());

        let mut sms_invite_data = match DatabaseService::read(ReadItem {
            key: invite_db_key.clone(),
            cf: SEND_INVITE_SMS_MESSAGES_CONFIG_KEY,
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

        let now = Utc::now().timestamp_millis() as u64;

        // todo: move the 7 days to the verifier config so it is configurable
        if i64::abs(now as i64 - sms_invite_data.last_message_sent_time_stamp as i64)
            < Duration::days(7).num_milliseconds()
        {
            info!("Last sms sent less than 1 week ago - skipping invite");
            return Ok(());
        }

        if sms_invite_data.messages_sent >= max_invites_per_number as u32 {
            info!("Already sent the max invites to this number - skipping invite");
            return Ok(());
        }

        // todo: send the invite here

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
            cf: SEND_INVITE_SMS_MESSAGES_CONFIG_KEY,
            ttl: 0,
        })
        .await?;

        Ok(())
    }
}
