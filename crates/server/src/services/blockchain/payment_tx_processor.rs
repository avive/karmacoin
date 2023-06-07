// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::tokenomics::Tokenomics;
use crate::services::db_config_service::{
    MOBILE_NUMBERS_COL_FAMILY, TRANSACTIONS_COL_FAMILY, USERS_COL_FAMILY,
};
use crate::services::push_notes::{send_tx_push_note, PaymentTxPushNotesData};
use anyhow::{anyhow, Result};
use base::genesis_config_service::{AMBASSADOR_CHAR_TRAIT_ID, SPENDER_CHAR_TRAIT_ID};
use base::hex_utils::{hex_string, short_hex_string};
use base::karma_coin::karma_coin_core_types::{
    CommunityMembership, ExecutionResult, FeeType, PaymentTransactionV1, SignedTransaction,
    TransactionBody, TransactionEvent, TransactionType, User,
};
use base::karma_coin_format::format_kc_amount;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use std::collections::HashMap;

impl BlockChainService {
    /// Get payee User from chain from the body of a payment transaction
    pub(crate) async fn get_payee_user_from_tx_body(
        tx_body: &TransactionBody,
    ) -> Result<Option<User>> {
        let payment_tx: PaymentTransactionV1 = tx_body.get_payment_transaction_v1()?;

        // find payee account id by phone number of from tx to_account_id field
        let payee_account_id = match payment_tx.to_number {
            Some(to_number) => {
                let mobile_number: String = to_number.number;

                // locate payee's account Id by mobile number form the index
                // note that this index always have the last created account with this phone number
                let payee_account_id_data = DatabaseService::read(ReadItem {
                    key: Bytes::from(mobile_number.as_bytes().to_vec()),
                    cf: MOBILE_NUMBERS_COL_FAMILY,
                })
                .await?;

                if payee_account_id_data.is_none() {
                    return Ok(None);
                }

                payee_account_id_data.unwrap().0.as_ref().to_vec()
            }
            None => payment_tx.to_account_id.as_ref().unwrap().data.clone(),
        };

        let payee_user_data = DatabaseService::read(ReadItem {
            key: Bytes::from(payee_account_id.clone()),
            cf: USERS_COL_FAMILY,
        })
        .await?;

        if payee_user_data.is_none() {
            warn!("payee account not found on chain");
            return Ok(None);
        }

        Ok(Some(User::decode(payee_user_data.unwrap().0.as_ref())?))
    }

    /// Get the on-chain User for the tx payee for a payment transaction
    /// Returns None if no user exists for the payee's mobile number provided in the tx
    /// or if there is no onchain account for the tx provided account_to id
    pub(crate) async fn get_payee_user(
        signed_transaction: &SignedTransaction,
    ) -> Result<Option<User>> {
        let tx_body = signed_transaction.get_body()?;
        BlockChainService::get_payee_user_from_tx_body(&tx_body).await
    }

    /// Process a user to user appreciation part of a payment transaction
    fn process_community_appreciation(
        &mut self,
        payer: &mut User,
        payee: &mut User,
        payment_tx: &PaymentTransactionV1,
        event: &mut TransactionEvent,
    ) {
        // leadaerboard processing

        // community appreciation processing
        let community_id = payment_tx.community_id;
        if community_id == 0 {
            // handle standard appreciation w/o a community context - assign trait and update score
            payee.inc_trait_score(payment_tx.char_trait_id, 0);
            payee.karma_score += 1;
            event.appreciation_char_trait_idx = payment_tx.char_trait_id;
            event.appreciation_community_id = 0;
            return;
        }

        if let Some(payer_membership) = payer.get_community_membership(community_id) {
            // Payer is member of the community - check if payee is a member
            let payee_membership = payee.get_community_membership(community_id);

            if payee_membership.is_none() && payer_membership.is_admin {
                info!("payer is admin and payee is not a member - creating community membership");
                let new_membership = CommunityMembership {
                    community_id,
                    is_admin: false,
                    karma_score: 1, // payee gets 1 karma point for joining the community
                };
                payee.community_memberships.push(new_membership);
            }

            // get updated membership status
            let payee_membership = payee.get_community_membership(community_id);

            if payee_membership.is_none() {
                info!("payer is not admin in community and payee is not a already member - disregard this community appreciation");
                return;
            }

            // payee is now a member - add 1 karma point in the community for the received appreciation
            payee_membership.unwrap().karma_score += 1;
            payee.inc_trait_score(payment_tx.char_trait_id, community_id);
            // give payer 1 karma score in the community for the sent appreciation
            payer_membership.karma_score += 1;
            event.appreciation_char_trait_idx = payment_tx.char_trait_id;
            event.appreciation_community_id = payment_tx.community_id;
        } else {
            info!("Payer non a member of the community - disregard this community appreciation");
        }
    }

    /// Process a payment transaction from payer to payee - update ledger state, emit tx event
    /// This is a helper method for the block creator and is used as part of block creation flow
    pub(crate) async fn process_payment_transaction(
        &mut self,
        signed_transaction: &SignedTransaction,
        payer: &mut User,
        payee: &mut User,
        sign_ups: &mut HashMap<Vec<u8>, SignedTransaction>,
        tokenomics: &Tokenomics,
        event: &mut TransactionEvent,
    ) -> Result<()> {
        let tx_hash = signed_transaction.get_hash()?;

        info!(
            "Processing payment transaction with hash {}",
            short_hex_string(tx_hash.as_ref())
        );

        // reject a payment from user to itself
        if payer.account_id.as_ref().unwrap().data == payee.account_id.as_ref().unwrap().data {
            return Err(anyhow!("You can't send karma coins to yourself"));
        }

        // validate the transaction
        signed_transaction.validate().await?;
        let tx_body = signed_transaction.get_body()?;

        // validate tx body and user nonce
        tx_body.validate(payer.nonce).await?;

        info!("Processing payment transaction: {}", signed_transaction);
        info!("Body: {}", tx_body);
        info!("From user: {}", payer);
        info!("To user: {}", payee);

        let payment_tx: PaymentTransactionV1 = tx_body.get_payment_transaction_v1()?;
        payment_tx.verify_syntax()?;

        if payer.account_id.as_ref().unwrap().data != payment_tx.from.as_ref().unwrap().data {
            return Err(anyhow!(
                "From account in payment tx must be the same as the signer account "
            ));
        }

        info!("Payment data: {}", payment_tx);

        let payment_amount = payment_tx.amount;

        let apply_subsidy = tokenomics
            .should_subsidise_transaction_fee(0, tx_body.fee, TransactionType::PaymentV1)
            .await?;

        info!("fee subsidised applied: {}", apply_subsidy);

        // actual fee amount to be paid by the user. 0 if fee is subsidised by the protocol
        let user_tx_fee_amount = if apply_subsidy { 0 } else { tx_body.fee };

        let fee_type = if apply_subsidy {
            FeeType::Mint
        } else {
            FeeType::User
        };

        if payer.balance < payment_amount + user_tx_fee_amount {
            // we reject the transaction and don't mint tx fee subsidy in this case
            // to avoid spamming the network with txs with insufficient funds
            return Err(anyhow!("payer has insufficient balance to pay"));
        }

        // update payee balance to reflect payment and tx fee (when applicable)
        payee.balance += payment_amount;

        info!("user paid tx fee: {}", user_tx_fee_amount);

        info!("payer balance before tx: {}", payer.balance);

        // update payer balance to reflect payment
        payer.balance -= payment_amount + user_tx_fee_amount;

        info!("payer balance after tx: {}", payer.balance);

        if payment_tx.char_trait_id != 0 {
            self.process_community_appreciation(payer, payee, &payment_tx, event);
        } else {
            // payment transaction w/o an appreciation
            // payer gets 1 point in spender char trait and in karma score
            payer.inc_trait_score(SPENDER_CHAR_TRAIT_ID, 0);
            payer.karma_score += 1;
        }

        let referral_reward_amount = tokenomics.get_referral_reward_amount().await?;

        let mut referral_reward_awarded = false;
        if let Some(payee_number) = payment_tx.to_number {
            let number = payee_number.number;
            // apply new user referral reward to the payer if applicable
            if sign_ups.contains_key(number.as_bytes()) {
                // remove from signups map to prevent double referral rewards for for the same new user
                sign_ups.remove(number.as_bytes());

                // this is a new user referral payment tx - payer should get the referral fee!
                //let _sign_up_tx = sign_ups.get(mobile_number.as_bytes()).unwrap();
                // todo: award signer with the referral reward if applicable
                info!(
                    "apply referral reward amount: {} to: {}",
                    referral_reward_amount, payer.user_name
                );
                payer.balance += referral_reward_amount;

                // Give payer the ambassador trait and 1 karma point for helping to grow the network
                payer.inc_trait_score(AMBASSADOR_CHAR_TRAIT_ID, 0);
                payer.karma_score += 1;
                referral_reward_awarded = true;
            };
        }

        // Give payer karma points for spending karma coins
        payer.inc_trait_score(SPENDER_CHAR_TRAIT_ID, 0);
        payer.karma_score += 1;

        // add user to leaderboard only if karma rewards are still allocated
        // and user is eligible for a reward
        if tokenomics.get_karma_coin_reward_amount().await? > 0
            && payer.is_eligible_for_karma_reward()
        {
            info!("Adding payer to leaderboard");
            // update leader board for an appreciation
            self.leader_board_upsert(payer, payment_tx.char_trait_id)
                .await?;
        }

        // update the user's nonce to the tx nonce
        info!("setting user nonce to {}", payer.nonce + 1);
        payer.nonce += 1;

        // index the transaction in the db by hash
        let mut tx_data = Vec::with_capacity(signed_transaction.encoded_len());
        info!(
            "binary transaction size: {}",
            signed_transaction.encoded_len()
        );

        signed_transaction.encode(&mut tx_data)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: tx_hash.clone(),
                value: Bytes::from(tx_data),
            },
            cf: TRANSACTIONS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // index the transaction in the db for both payer and payee
        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(payer.account_id.as_ref().unwrap().data.to_vec()),
        )
        .await?;

        self.index_transaction_by_account_id(
            signed_transaction,
            Bytes::from(payee.account_id.as_ref().unwrap().data.to_vec()),
        )
        .await?;

        // Update payer on chain account
        let mut buf = Vec::with_capacity(payer.encoded_len());
        payer.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(payer.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // Update payee on chain account
        let mut buf = Vec::with_capacity(payee.encoded_len());
        payee.encode(&mut buf)?;
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: Bytes::from(payee.account_id.as_ref().unwrap().data.to_vec()),
                value: Bytes::from(buf),
            },
            cf: USERS_COL_FAMILY,
            ttl: 0,
        })
        .await?;

        // update tx event
        event.referral_reward = referral_reward_amount;
        event.fee_type = fee_type as i32;
        event.fee = tx_body.fee;
        event.result = ExecutionResult::Executed as i32;

        // handle push notes
        if referral_reward_awarded {
            info!("Send push note about referral reward to payer - todo: implement me");
            // this was payment on signup and payer got referral reward
            // send payer push note about it - no need to send push to payee
        } else {
            // send a push note to payee about the push
            // todo: add community info when applicable to make it more personalized to community
            use data_encoding::BASE64;

            let to_id = BASE64.encode(payee.account_id.as_ref().unwrap().data.as_ref());
            let amount = format_kc_amount(payment_amount);
            let data = PaymentTxPushNotesData {
                tx_id: hex_string(tx_hash.as_ref()),
                amount,
                to_id,
                char_id: payment_tx.char_trait_id,
                // todo: get emoji for chart_trait if it is non zero
                emoji: "".to_string(),
            };

            // don't fail operation if push note fails
            match send_tx_push_note(data).await {
                Ok(_) => info!("sent push note to payee"),
                Err(e) => error!("failed to send push note to payee: {}", e),
            }
        }

        Ok(())
    }
}
