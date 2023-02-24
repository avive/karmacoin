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

/// Signup reward in KCents in phase 1
pub const SIGNUP_REWARD_AMOUNT_PHASE1_KEY: &str = "signup_reward_p1_key";

/// Total rewards for phase 1
pub const SIGNUP_REWARD_ALLOCATION_PHASE1_KEY: &str = "signup_reward_alloc_p1_key";

/// Signup reward in KCents in phase 2. eligibility
pub const SIGNUP_REWARD_AMOUNT_PHASE2_KEY: &str = "signup_reward_p2_key";

/// Max number of signup rewards for phase 2
pub const SIGNUP_REWARD_ALLOCATION_PHASE2_KEY: &str = "signup_reward_alloc_p2";

/// Referral reward in KCents in phase 3
pub const SIGNUP_REWARD_AMOUNT_PHASE3_KEY: &str = "signup_reward_p3";

/// Referral reward in KCents in phase 1
pub const REFERRAL_REWARD_AMOUNT_PHASE1_KEY: &str = "referral_reward_p1";

/// Total referral rewards for phase 1
pub const REFERRAL_REWARD_ALLOCATION_PHASE1_KEY: &str = "referral_reward_alloc_p1";

/// Referral reward in KCents in phase 2
pub const REFERRAL_REWARD_AMOUNT_PHASE2_KEY: &str = "referral_reward_p2";

/// Total rewards for phase 2
pub const REFERRAL_REWARD_ALLOCATION_PHASE2_KEY: &str = "referral_reward_alloc_p2";

/// Total tx fee subsidies phase 1
pub const TX_FEE_SUBSIDY_MAX_AMOUNT_PHASE1_KEY: &str = "tx_fee_subsidy_max_amount_p1";

/// Total tx fee subsidies phase 1
pub const TX_FEE_SUBSIDY_ALLOCATION_PHASE1_KEY: &str = "tx_fee_subsidy_allocation_p1";

/// Max subsided transactions per user
pub const TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY: &str = "tx_fee_subsidy_max_txs_per_user";

/// The Max tx fee amount that the protocol should subsidise after phase 1 allocation is exhausted
pub const TX_FEE_SUBSIDY_MAX_AMOUNT_KEY: &str = "tx_fee_subsidy_max_amount";

/// Causes reward per period0
pub const CAUSES_REWARD_AMOUNT_PER_PERIOD: &str = "causes_reward_amount_per_period";

/// Causes allocation period. e.g. every 4 weeks
pub const CAUSES_REWARD_WEEKS_PERIOD: &str = "causes_reward_weeks_period";

/// Number of causes to reward each period
pub const CAUSES_PER_PERIOD: &str = "causes_per_period";

/// Total Kcs allocated for causes rewards
pub const CAUSES_REWARDS_ALLOCATION: &str = "causes_rewards_allocation";

/// last block alienable for block reward
pub const BLOCK_REWARDS_LAST_BLOCK: &str = "block_rewards_last_block";

/// Block reward amount
pub const BLOCK_REWARDS_AMOUNT: &str = "block_rewards_amount";

/// Karma reward amount in KCents
pub const KARMA_REWARD_AMOUNT: &str = "karma_reward_amount";

/// Karma rewards period
pub const KARMA_REWARD_PERIOD: &str = "karma_reward_period";

/// Number of users to get rewarded each period
pub const KARMA_REWARD_TOP_N_USERS_KEY: &str = "karma_reward_top_n_users";

/// Karma rewards allocation
pub const KARAM_REWARDS_ALLOCATION_KEY: &str = "karma_rewards_allocation";

/// Treasury account id
pub const TREASURY_ACCOUNT_ID_KEY: &str = "treasury_account_id";

/// Treasury account name
pub const TREASURY_ACCOUNT_NAME_KEY: &str = "treasury";

/// Treasury pre-minted amount in KCents
pub const TREASURY_PREMINT_COINS_AMOUNT_KEY: &str = "treasury_premint_coins";

/// A set of canonical mobile phone verifiers accounts ids
pub const VERIFIERS_ACCOUNTS_IDS: &str = "verifiers_accounts_ids";

/// This is the signup trait - user gets it for signing up
pub const SIGNUP_CHAR_TRAIT_ID: u32 = 1;

/// User gets a point in this trait for each sent appreciation / payment
pub const SPENDER_CHAR_TRAIT_ID: u32 = 2;

/// User get
/// s one for each referral who signed up
pub const AMBASSADOR_CHAR_TRAIT_ID: u32 = 41;

/// This must be true across all traits defined in genesis configs
pub const NO_CHAR_TRAIT_ID: u32 = 0;

/// This service handles the kc blockchain genesis configuration
/// It provides default values for development, and merges in values from
/// a genesis config file when applicable
#[derive(Default)]
pub struct GenesisConfigService {
    config: Config,
    config_file: Option<String>,
    pub(crate) genesis_data: Option<GetGenesisDataResponse>,
    char_traits: Option<Vec<CharTrait>>,
}

#[async_trait::async_trait]
impl Actor for GenesisConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("starting...");

        // default supported char traits
        // todo: update based on canonical list of traits
        self.char_traits = Some(vec![
            // no appreciation is index 0
            CharTrait::new(0, "".into(), "".into()),
            // user gets 1 in this trait for signing up
            CharTrait::new(1, "a Karma Grower".into(), "😇".into()),
            // User gets this for every sent transaction / appreciation
            CharTrait::new(2, "a Karma Spender".into(), "🖖".into()),
            CharTrait::new(3, "Kind".into(), "😀".into()),
            CharTrait::new(4, "Helpful".into(), "😇".into()),
            CharTrait::new(5, "an Uber Geek".into(), "🖖".into()),
            CharTrait::new(6, "Awesome".into(), "😎".into()),
            CharTrait::new(7, "Smart".into(), "🥸".into()),
            CharTrait::new(8, "Sexy".into(), "😍".into()),
            CharTrait::new(9, "Patient".into(), "😀".into()),
            CharTrait::new(10, "Grateful".into(), "😇".into()),
            CharTrait::new(11, "Spiritual".into(), "🖖".into()),
            CharTrait::new(12, "Funny".into(), "😎".into()),
            CharTrait::new(13, "Caring".into(), "🥸".into()),
            CharTrait::new(14, "Loving".into(), "😍".into()),
            CharTrait::new(15, "Generous".into(), "😀".into()),
            CharTrait::new(16, "Honest".into(), "😇".into()),
            CharTrait::new(17, "Respectful".into(), "🖖".into()),
            CharTrait::new(18, "Creative".into(), "😎".into()),
            CharTrait::new(19, "Intelligent".into(), "🥸".into()),
            CharTrait::new(20, "Loyal".into(), "😍".into()),
            CharTrait::new(21, "Trustworthy".into(), "😀".into()),
            CharTrait::new(22, "Humble".into(), "😇".into()),
            CharTrait::new(23, "Courageous".into(), "🖖".into()),
            CharTrait::new(24, "Confident".into(), "😎".into()),
            CharTrait::new(25, "Passionate".into(), "🥸".into()),
            CharTrait::new(26, "Optimistic".into(), "😍".into()),
            CharTrait::new(27, "Adventurous".into(), "😀".into()),
            CharTrait::new(28, "Determined".into(), "😇".into()),
            CharTrait::new(29, "Selfless".into(), "🖖".into()),
            CharTrait::new(30, "Self-aware".into(), "😎".into()),
            CharTrait::new(31, "Self-disciplined".into(), "🥸".into()),
            CharTrait::new(32, "Mindful".into(), "😍".into()),
            CharTrait::new(33, "My Guardian Angel".into(), "😍".into()),
            CharTrait::new(34, "a Fairy".into(), "😍".into()),
            CharTrait::new(35, "a Wizard".into(), "😍".into()),
            CharTrait::new(36, "a Witch".into(), "😍".into()),
            CharTrait::new(37, "a Warrior".into(), "😍".into()),
            CharTrait::new(38, "a Healer".into(), "😍".into()),
            CharTrait::new(39, "a Guardian".into(), "😍".into()),
            CharTrait::new(40, "an Inspiration".into(), "😍".into()),
            // user gets 1 in this trait for each referral who signed up
            CharTrait::new(40, "a Karma Ambassador".into(), "😍".into()),
        ]);

        // default supported verifiers
        let verifiers: HashMap<String, String> = map! {
            "verifier1".into() => "a885bf7ac670b0f01a3551740020e115641005a93f59472002bfd1dc665f4a4e".into(),
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
            //
            // 10 KC per signup in phase 1
            .set_default(SIGNUP_REWARD_AMOUNT_PHASE1_KEY, 10_000_000)
            .unwrap()
            // 100m KCs allocated for signup rewards phase 1
            .set_default(SIGNUP_REWARD_ALLOCATION_PHASE1_KEY, 100_000_000)
            .unwrap()
            // Signup phase 2 rewards amount - 1 KC
            .set_default(SIGNUP_REWARD_AMOUNT_PHASE2_KEY, 1_000_000)
            .unwrap()
            // phase 2 rewards amount allocation
            .set_default(SIGNUP_REWARD_ALLOCATION_PHASE2_KEY, 200_000_000)
            .unwrap()
            // Phase 3 reward amount per signup - 1000 KCents
            .set_default(SIGNUP_REWARD_AMOUNT_PHASE3_KEY, 1000)
            .unwrap()
            //
            // phase 1 reward amount per referral - 10 KC
            .set_default(REFERRAL_REWARD_AMOUNT_PHASE1_KEY, 10_000_000)
            .unwrap()
            // phase 1 referral rewards allocation - 100M Kcs
            .set_default(REFERRAL_REWARD_ALLOCATION_PHASE1_KEY, 100_000_000)
            .unwrap()
            // phase 2 referral reward amount - 1 KC
            .set_default(REFERRAL_REWARD_AMOUNT_PHASE2_KEY, 10_000_000)
            .unwrap()
            // phase 2 referral rewards allocation - 200M Kcs
            .set_default(REFERRAL_REWARD_ALLOCATION_PHASE2_KEY, 200_000_000)
            .unwrap()
            //
            // Last block eligible for block rewards
            .set_default(BLOCK_REWARDS_LAST_BLOCK, 500_000_000)
            .unwrap()
            // The block reward constant amount
            .set_default(BLOCK_REWARDS_AMOUNT, 1_000_000)
            .unwrap()
            //
            // Karma rewards amount per user - 10 KC
            .set_default(KARMA_REWARD_AMOUNT, 10_000_000)
            .unwrap()
            // Karma rewards computation period in weeks
            .set_default(KARMA_REWARD_PERIOD, 4)
            .unwrap()
            // The top 1000 users who didn't get karma reward are eligible every period
            .set_default(KARMA_REWARD_TOP_N_USERS_KEY, 1_000)
            .unwrap()
            // karma rewards allocation - 300M KCs
            .set_default(KARAM_REWARDS_ALLOCATION_KEY, 300_000_000)
            .unwrap()
            //
            // The max amount for a tx fee subsidy - 1 KCent
            .set_default(TX_FEE_SUBSIDY_MAX_AMOUNT_PHASE1_KEY, 1)
            .unwrap()
            // The max amount for a tx fee subsidy after phase 1 - 1 KCent
            .set_default(TX_FEE_SUBSIDY_MAX_AMOUNT_KEY, 1)
            .unwrap()
            // The amount of coins allocated for phase 1 tx fees - 250M KCs
            .set_default(TX_FEE_SUBSIDY_ALLOCATION_PHASE1_KEY, 250_000_000)
            .unwrap()
            // The max number of txs that can be subsidised per user
            .set_default(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY, 10)
            .unwrap()
            //
            // The period in weeks in which causes rewards are calculated
            .set_default(CAUSES_REWARD_WEEKS_PERIOD, 4)
            .unwrap()
            // The number of causes rewarded in each period
            .set_default(CAUSES_PER_PERIOD, 20)
            .unwrap()
            // Total coin allocated for causes rewards - 225M KCs
            .set_default(CAUSES_REWARDS_ALLOCATION, 225_000_000)
            .unwrap()
            //
            //
            .set_default(VERIFIERS_ACCOUNTS_IDS, verifiers)
            .unwrap()
            .set_default(TREASURY_PREMINT_COINS_AMOUNT_KEY, 5_000_000)
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

            signup_reward_phase1_alloc: self.config.get_int(SIGNUP_REWARD_ALLOCATION_PHASE1_KEY)?
                as u64,
            signup_reward_phase2_alloc: self.config.get_int(SIGNUP_REWARD_ALLOCATION_PHASE2_KEY)?
                as u64,
            signup_reward_phase1_amount: self.config.get_int(SIGNUP_REWARD_AMOUNT_PHASE1_KEY)?
                as u64,
            signup_reward_phase2_amount: self.config.get_int(SIGNUP_REWARD_AMOUNT_PHASE2_KEY)?
                as u64,
            signup_reward_phase3_start: self.config.get_int(SIGNUP_REWARD_AMOUNT_PHASE3_KEY)?
                as u64,

            referral_reward_phase1_alloc: self
                .config
                .get_int(REFERRAL_REWARD_ALLOCATION_PHASE1_KEY)?
                as u64,
            referral_reward_phase2_alloc: self
                .config
                .get_int(REFERRAL_REWARD_ALLOCATION_PHASE2_KEY)?
                as u64,
            referral_reward_phase1_amount: self.config.get_int(REFERRAL_REWARD_AMOUNT_PHASE1_KEY)?
                as u64,
            referral_reward_phase2_amount: self.config.get_int(REFERRAL_REWARD_AMOUNT_PHASE2_KEY)?
                as u64,

            tx_fee_subsidy_max_per_user: self.config.get_int(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY)?
                as u64,
            tx_fee_subsidies_alloc: self.config.get_int(TX_FEE_SUBSIDY_ALLOCATION_PHASE1_KEY)?
                as u64,
            tx_fee_subsidy_max_amount: self.config.get_int(TX_FEE_SUBSIDY_MAX_AMOUNT_KEY)? as u64,

            block_reward_amount: self.config.get_int(BLOCK_REWARDS_AMOUNT)? as u64,
            block_reward_last_block: self.config.get_int(BLOCK_REWARDS_LAST_BLOCK)? as u64,

            karma_reward_amount: self.config.get_int(KARMA_REWARD_AMOUNT)? as u64,
            karma_reward_alloc: self.config.get_int(KARAM_REWARDS_ALLOCATION_KEY)? as u64,
            karma_reward_top_n_users: self.config.get_int(KARMA_REWARD_TOP_N_USERS_KEY)? as u64,

            treasury_premint_amount: self.config.get_int(TREASURY_PREMINT_COINS_AMOUNT_KEY)? as u64,
            treasury_account_id: self.config.get_string(TREASURY_ACCOUNT_ID_KEY)?,
            treasury_account_name: self.config.get_string(TREASURY_ACCOUNT_NAME_KEY)?,

            verifiers: self.get_verifiers().await?,

            char_traits: self.char_traits.as_ref().unwrap().clone(),
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
