// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use map_macro::map;

use crate::karma_coin::karma_coin_api::{GetGenesisDataRequest, GetGenesisDataResponse};
use crate::karma_coin::karma_coin_core_types::{AccountId, CharTrait, PhoneVerifier};
use config::{Config, Environment, Map, Value};
use log::*;
use std::collections::HashMap;
use xactor::*;

// Blockchain network id
pub const NET_ID_KEY: &str = "net_id_key";
pub const NET_NAME_KEY: &str = "net_name_key";

pub const DEVNET_ID: u32 = 1;
pub const DEVNET_NAME: &str = "devnet";

// consensus genesis time in milliseconds
pub const GENESIS_TIMESTAMP_MS_KEY: &str = "genesis_timestamp_key";

// Default tx fee amount
pub const DEF_TX_FEE_KEY: &str = "def_tx_fee_key";

/// Signup reward in KCents in phase 2
pub const CHAR_TRAITS_KEY: &str = "char_traits_key";

/// Signup reward in KCents in phase 1
pub const SIGNUP_REWARD_PHASE1_KEY: &str = "signup_reward_p1_key";

/// Max number of rewards for phase 1
pub const SIGNUP_REWARD_PHASE1_ALLOCATION_KEY: &str = "signup_reward_alloc_p1_key";

/// Signup reward in KCents in phase 2
pub const SIGNUP_REWARD_PHASE2_KEY: &str = "signup_reward_p2_key";

/// Max number of signup rewards for phase 2
pub const SIGNUP_REWARD_PHASE2_ALLOCATION_KEY: &str = "signup_reward_alloc_p2";

/// Referral reward in KCents in phase 3
pub const SIGNUP_REWARD_PHASE3_KEY: &str = "signup_reward_p3";

/// Referral reward in KCents in phase 1
pub const REFERRAL_REWARD_PHASE1_KEY: &str = "referral_reward_p1";

/// Max number of referral rewards for phase 1
pub const REFERRAL_REWARD_PHASE1_ALLOCATION_KEY: &str = "referral_reward_alloc_p1";

/// Referral reward in KCents in phase 2
pub const REFERRAL_REWARD_PHASE2_KEY: &str = "referral_reward_p2";

/// Max number of referral rewards for phase 2
pub const REFERRAL_REWARD_PHASE2_ALLOCATION_KEY: &str = "referral_reward_alloc_p2";

/// Total max number of tx fee subsidies
pub const TX_FEE_SUBSIDY_ALLOCATION_KEY: &str = "tx_fee_subsidy_total";

/// Max subsided transactions per user
pub const TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY: &str = "tx_fee_subsidy_max_txs_per_user";

/// The Max tx fee amount that the protocol should subsidise
pub const TX_FEE_SUBSIDY_MAX_AMOUNT: &str = "tx_fee_subsidy_max_amount";

/// last block alienable for block reward
pub const BLOCK_REWARDS_LAST_BLOCK: &str = "block_rewards_last_block";

/// Block reward amount
pub const BLOCK_REWARDS_AMOUNT: &str = "block_rewards_amount";

/// Karma reward amount in KCents
pub const KARMA_REWARD_AMOUNT: &str = "karma_reward_amount";

/// Number of users to get rewarded each period
pub const KARMA_REWARD_TOP_N_USERS_KEY: &str = "karma_reward_top_n_users";

/// Max number of karma rewards
pub const KARAM_REWARDS_ALLOCATION_KEY: &str = "karma_rewards_allocation";

/// Treasury account id
pub const TREASURY_ACCOUNT_ID_KEY: &str = "treasury_account_id";

/// Treasury account name
pub const TREASURY_ACCOUNT_NAME_KEY: &str = "treasury";

/// Treasury pre-minted amount in KCents
pub const TREASURY_PREMINT_COINS_AMOUNT_KEY: &str = "treasury_premint_coins";

/// A set of canonical mobile phone verifiers accounts ids
pub const VERIFIERS_ACCOUNTS_IDS: &str = "verifiers_accounts_ids";

/// This must be true across all traits defined in genesis configs
pub const KARMA_COIN_AMBASSADOR_CHAR_TRAIT_ID: u32 = 1;

/// This must be true across all traits defined in genesis configs
pub const KARMA_COIN_SPENDER_CHAR_TRAIT_ID: u32 = 2;

/// This service handles the kc blockchain genesis configuration
/// It provides default values for development, and merges in values from
/// a genesis config file when applicable
#[derive(Default)]
pub struct GenesisConfigService {
    config: Config,
    config_file: Option<String>,
    pub(crate) genesis_data: Option<GetGenesisDataResponse>,
}

#[async_trait::async_trait]
impl Actor for GenesisConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("starting...");

        // default supported char traits
        let char_traits: HashMap<String, String> = map! {
            "0".into() => "None".into(),
            // note that this must remain constant in all genesis configs:
            "1".into() => "KarmaCoin Ambassador".into(),
            "2".into() => "KarmaCoin Spender".into(),
            "3".into() => "Kind".into(),
            "4".into() => "Smart".into(),
            "5".into() => "Sexy".into(),
        };

        // default supported verifiers
        let verifiers: HashMap<String, String> = map! {
            "verifier1".into() => "ec3d84d8e7ded4d438b67eae89ce3fb94c8d77fe0816af797fc40c9a6807a5cd".into(),
            "verifier2".into() => "f0d0c0b0a090807060504030201000f0e0d0c0b0a090807060504030201000f".into(),
        };

        let builder = Config::builder();
        // Set defaults and merge genesis config file to overwrite
        let config = builder
            .set_default(NET_ID_KEY, DEVNET_ID)
            .unwrap()
            .set_default(NET_NAME_KEY, DEVNET_NAME)
            .unwrap()
            .set_default(GENESIS_TIMESTAMP_MS_KEY, 1672860236)
            .unwrap()
            .set_default(DEF_TX_FEE_KEY, 100)
            .unwrap()
            .set_default(CHAR_TRAITS_KEY, char_traits)
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE1_KEY, 10 * (10 ^ 6))
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE1_ALLOCATION_KEY, 40 * (10 ^ 6))
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE2_KEY, 10 ^ 6)
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE2_ALLOCATION_KEY, 100 * (10 ^ 6))
            .unwrap()
            // Phase 3 rewards post the phase 2 allocated number of users
            .set_default(SIGNUP_REWARD_PHASE3_KEY, 1000)
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE1_KEY, 10 * (10 ^ 6))
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE1_ALLOCATION_KEY, 40 * (10 ^ 6))
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE2_KEY, 10 ^ 6)
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE2_ALLOCATION_KEY, 100 * (10 ^ 6))
            .unwrap()
            // Last block eligible for block rewards
            .set_default(BLOCK_REWARDS_LAST_BLOCK, 500 * (10 ^ 6))
            .unwrap()
            // The block reward constant amount
            .set_default(BLOCK_REWARDS_AMOUNT, 10 ^ 6)
            .unwrap()
            .set_default(KARMA_REWARD_AMOUNT, 10 * (10 ^ 6))
            .unwrap()
            .set_default(KARMA_REWARD_TOP_N_USERS_KEY, 1_000)
            .unwrap()
            .set_default(KARAM_REWARDS_ALLOCATION_KEY, 250 * (10 ^ 6))
            .unwrap()
            .set_default(TX_FEE_SUBSIDY_MAX_AMOUNT, 1000)
            .unwrap()
            .set_default(TX_FEE_SUBSIDY_ALLOCATION_KEY, 250 * (10 ^ 6))
            .unwrap()
            .set_default(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY, 10)
            .unwrap()
            .set_default(VERIFIERS_ACCOUNTS_IDS, verifiers)
            .unwrap()
            .set_default(TREASURY_PREMINT_COINS_AMOUNT_KEY, 5 * (10 ^ 6))
            .unwrap()
            // todo: replace it with 3 accounts with 3 different keys
            .set_default(
                TREASURY_ACCOUNT_ID_KEY,
                "ec3d84d8e7ded4d438b67eae89ce3fb94c8d77fe0816af797fc40c9a6807a5cd",
            )
            .unwrap()
            .set_default(TREASURY_ACCOUNT_NAME_KEY, "treasury")
            .unwrap()
            .add_source(
                Environment::with_prefix("GENESIS")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()
            .unwrap();

        // load configs from file if it was set
        if let Some(config_file) = &self.config_file {
            #[allow(deprecated)]
            self.config
                .merge(config::File::with_name(config_file))
                .unwrap();
        }

        self.config = config;

        Ok(())
    }
}

impl Service for GenesisConfigService {}

// helpers
impl GenesisConfigService {
    /// Returns all supported char traits from genesis data
    async fn get_verifiers(&mut self) -> Result<Vec<PhoneVerifier>> {
        let mut verifiers = vec![];
        for (name, account_id) in self.config.get_table(VERIFIERS_ACCOUNTS_IDS).unwrap() {
            verifiers.push(PhoneVerifier {
                account_id: Some(AccountId {
                    data: account_id.into_string()?.as_bytes().to_vec(),
                }),
                name,
            })
        }

        Ok(verifiers)
    }

    /// Returns all supported char traits from genesis data
    async fn get_char_traits(&mut self) -> Result<Vec<CharTrait>> {
        let mut traits = vec![];
        for (id, name) in self.config.get_table(CHAR_TRAITS_KEY).unwrap() {
            traits.push(CharTrait::new(
                id.parse().unwrap(),
                name.into_string().unwrap().as_str(),
            ));
        }

        Ok(traits)
    }

    pub async fn get(key: String) -> Result<Option<String>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetValue(key)).await?;
        Ok(res)
    }

    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    pub async fn get_map(key: String) -> Result<Option<Map<String, Value>>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetMap(key)).await?;
        Ok(res)
    }

    pub async fn get_array(key: String) -> Result<Option<Vec<Value>>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetArray(key)).await?;
        Ok(res)
    }

    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }

    pub async fn set(key: String, value: String) -> Result<()> {
        let config = GenesisConfigService::from_registry().await?;
        config.call(SetValue { key, value }).await?
    }

    pub async fn set_bool(key: String, value: bool) -> Result<()> {
        let config = GenesisConfigService::from_registry().await?;
        config.call(SetBool { key, value }).await?
    }

    pub async fn set_u64(key: String, value: u64) -> Result<()> {
        let config = GenesisConfigService::from_registry().await?;
        config.call(SetU64 { key, value }).await?
    }
}

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

/// Merge content of genesis config file into the config
#[async_trait::async_trait]
impl Handler<SetConfigFile> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        // todo: verify config file exists and is readable by this process

        #[allow(deprecated)]
        self.config
            .merge(config::File::with_name(&msg.config_file))
            .unwrap();

        self.config_file = Some(msg.config_file.clone());

        info!(
            "Merging content of config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
    }
}

#[message(result = "Option<Map<String, Value>>")]
pub struct GetMap(pub String);

#[async_trait::async_trait]
impl Handler<GetMap> for GenesisConfigService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetMap,
    ) -> Option<Map<String, Value>> {
        match self.config.get_table(msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<Vec<Value>>")]
pub struct GetArray(pub String);

#[async_trait::async_trait]
impl Handler<GetArray> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetArray) -> Option<Vec<Value>> {
        match self.config.get_array(msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<bool>")]
pub struct GetBool(pub String);

#[async_trait::async_trait]
impl Handler<GetBool> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetBool) -> Option<bool> {
        match self.config.get_bool(msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<u64>")]
pub struct GetU64(pub String);

#[async_trait::async_trait]
impl Handler<GetU64> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetU64) -> Option<u64> {
        match self.config.get_int(msg.0.as_str()) {
            Ok(res) => Some(res as u64),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<String>")]
pub struct GetValue(pub String);

#[async_trait::async_trait]
impl Handler<GetValue> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetValue) -> Option<String> {
        match self.config.get_string(msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetValue {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
impl Handler<SetValue> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetValue) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetU64 {
    pub key: String,
    pub value: u64,
}

#[async_trait::async_trait]
impl Handler<SetU64> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetU64) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<GetGenesisDataResponse>")]
pub struct GetGenesisData {
    pub request: GetGenesisDataRequest,
}

#[async_trait::async_trait]
impl Handler<GetGenesisData> for GenesisConfigService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetGenesisData,
    ) -> Result<GetGenesisDataResponse> {
        if let Some(data) = self.genesis_data.as_ref() {
            return Ok(data.clone());
        }

        let genesis_data = GetGenesisDataResponse {
            net_id: self.config.get_int(NET_ID_KEY).unwrap() as u32,
            net_name: self.config.get_string(NET_NAME_KEY)?,
            genesis_time: self.config.get_int(GENESIS_TIMESTAMP_MS_KEY)? as u64,

            signup_reward_phase1_alloc: self.config.get_int(SIGNUP_REWARD_PHASE1_ALLOCATION_KEY)?
                as u64,
            signup_reward_phase2_alloc: self.config.get_int(SIGNUP_REWARD_PHASE2_ALLOCATION_KEY)?
                as u64,
            signup_reward_phase1_amount: self.config.get_int(SIGNUP_REWARD_PHASE1_KEY)? as u64,
            signup_reward_phase2_amount: self.config.get_int(SIGNUP_REWARD_PHASE2_KEY)? as u64,
            signup_reward_phase3_start: self.config.get_int(SIGNUP_REWARD_PHASE3_KEY)? as u64,

            referral_reward_phase1_alloc: self
                .config
                .get_int(REFERRAL_REWARD_PHASE1_ALLOCATION_KEY)?
                as u64,
            referral_reward_phase2_alloc: self
                .config
                .get_int(REFERRAL_REWARD_PHASE2_ALLOCATION_KEY)?
                as u64,
            referral_reward_phase1_amount: self.config.get_int(REFERRAL_REWARD_PHASE1_KEY)? as u64,
            referral_reward_phase2_amount: self.config.get_int(REFERRAL_REWARD_PHASE2_KEY)? as u64,

            tx_fee_subsidy_max_per_user: self.config.get_int(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY)?
                as u64,
            tx_fee_subsidies_alloc: self.config.get_int(TX_FEE_SUBSIDY_ALLOCATION_KEY)? as u64,
            tx_fee_subsidy_max_amount: self.config.get_int(TX_FEE_SUBSIDY_MAX_AMOUNT)? as u64,

            block_reward_amount: self.config.get_int(BLOCK_REWARDS_AMOUNT)? as u64,
            block_reward_last_block: self.config.get_int(BLOCK_REWARDS_LAST_BLOCK)? as u64,

            karma_reward_amount: self.config.get_int(KARMA_REWARD_AMOUNT)? as u64,
            karma_reward_alloc: self.config.get_int(KARAM_REWARDS_ALLOCATION_KEY)? as u64,
            karma_reward_top_n_users: self.config.get_int(KARMA_REWARD_TOP_N_USERS_KEY)? as u64,

            treasury_premint_amount: self.config.get_int(TREASURY_PREMINT_COINS_AMOUNT_KEY)? as u64,
            treasury_account_id: self.config.get_string(TREASURY_ACCOUNT_ID_KEY)?,
            treasury_account_name: self.config.get_string(TREASURY_ACCOUNT_NAME_KEY)?,

            char_traits: self.get_char_traits().await?,
            verifiers: self.get_verifiers().await?,
        };

        // cache genesis data as it is read-only
        self.genesis_data = Some(genesis_data.clone());

        Ok(genesis_data)
    }
}

#[message(result = "Result<()>")]
pub struct SetBool {
    pub key: String,
    pub value: bool,
}

#[async_trait::async_trait]
impl Handler<SetBool> for GenesisConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetBool) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
