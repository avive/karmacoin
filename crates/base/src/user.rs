// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::genesis_config_service::KARMA_REWARD_TRAIT_ID;
use crate::hex_utils::short_hex_string;
use crate::karma_coin::karma_coin_core_types::{
    AccountId, CommunityMembership, MobileNumber, TraitScore, User,
};
use anyhow::{anyhow, Result};
use log::info;
use std::fmt;
use std::fmt::{Display, Formatter};

impl User {
    /// Verify all fields
    pub fn verify_syntax(&self) -> Result<()> {
        if self.account_id.is_none() {
            return Err(anyhow!("account id is required"));
        }
        if self.mobile_number.is_none() {
            return Err(anyhow!("mobile number is required"));
        }
        Ok(())
    }
}

impl User {
    pub fn new(account_id: AccountId, user_name: String, mobile_number: MobileNumber) -> Self {
        User {
            account_id: Some(account_id),
            nonce: 0,
            user_name,
            mobile_number: Some(mobile_number),
            balance: 0,
            trait_scores: vec![],
            pre_keys: vec![],
            karma_score: 1, // new users have karma score of 1
            community_memberships: vec![],
        }
    }

    /// Returns community membership for a given community id
    pub fn get_community_membership(
        &mut self,
        community_id: u32,
    ) -> Option<&mut CommunityMembership> {
        self.community_memberships
            .iter_mut()
            .find(|community_membership| community_membership.community_id == community_id)
    }

    pub fn is_community_member(&self, community_id: u32) -> bool {
        self.community_memberships
            .iter()
            .any(|community_membership| community_membership.community_id == community_id)
    }

    /// Reruns score for a trait_id with optional community context
    pub fn get_trait_score(&self, trait_id: u32, community_id: u32) -> u32 {
        for trait_score in self.trait_scores.iter() {
            if trait_score.trait_id == trait_id && trait_score.community_id == community_id {
                return trait_score.score;
            }
        }

        0
    }

    /// Returns true if user is eligible for karma reward
    pub fn is_eligible_for_karma_reward(&self) -> bool {
        !self
            .trait_scores
            .iter()
            .any(|trait_score| trait_score.trait_id == KARMA_REWARD_TRAIT_ID)
    }

    /// Inc the trait score for a given trait
    pub fn inc_trait_score(&mut self, trait_id: u32, community_id: u32) {
        let mut found = false;
        for trait_score in self.trait_scores.iter_mut() {
            if trait_score.trait_id == trait_id && trait_score.community_id == community_id {
                trait_score.score += 1;
                found = true;
                info!(
                    "User name: {}, inc_trait_score: trait_id: {}, score: {}, community: {}",
                    self.user_name, trait_id, trait_score.score, community_id
                );
                break;
            }
        }

        if !found {
            self.trait_scores.push(TraitScore {
                trait_id,
                score: 1,
                community_id,
            });
            info!(
                "User name: {}, inc_trait_score: trait_id: {}, score: {}, community: {}",
                self.user_name, trait_id, 1, community_id,
            );
        }
    }
}

impl Display for User {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ name: {}, number: {}, account_id: {}, nonce: {}, \
            balance: {}, trait_scores: {:?}, pre_keys: {:?} }}",
            self.user_name,
            match self.mobile_number.as_ref() {
                Some(mobile_number) => mobile_number.number.to_string(),
                None => "[n/a]".to_string(),
            },
            short_hex_string(self.account_id.as_ref().unwrap().data.as_slice()),
            self.nonce,
            self.balance,
            self.trait_scores,
            self.pre_keys
        )
    }
}
