// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use config::{Config, Environment};
use log::*;
use xactor::*;

pub const NET_ID_KEY: &str = "net_id";
pub const DEF_TX_FEE_KEY : &str = "def_tx_fee";

/// Signup reward in KCents in phase 1
pub const SIGNUP_REWARD_PHASE1_KEY : &str = "signup_reward_p1";

/// Max number of rewards for phase 1
pub const SIGNUP_REWARD_PHASE1_ALLOCATION_KEY : &str = "signup_reward_alloc_p1";

/// Signup reward in KCents in phase 2
pub const SIGNUP_REWARD_PHASE2_KEY : &str = "signup_reward_p2";

/// Max number of signup rewards for phase 2
pub const SIGNUP_REWARD_PHASE2_ALLOCATION_KEY : &str = "signup_reward_alloc_p2";

/// Referral reward in KCents in phase 1
pub const REFERRAL_REWARD_PHASE1_KEY : &str = "referral_reward_p1";

/// Max number of referral rewards for phase 1
pub const REFERRAL_REWARD_PHASE1_ALLOCATION_KEY : &str = "referral_reward_alloc_p1";

/// Referral reward in KCents in phase 2
pub const REFERRAL_REWARD_PHASE2_KEY : &str = "referral_reward_p2";

/// Max number of referral rewards for phase 2
pub const REFERRAL_REWARD_PHASE2_ALLOCATION_KEY : &str = "referral_reward_alloc_p2";

/// Total number of tx fee subsidies
pub const TX_FEE_SUBSIDY_TOTAL_KEY: &str = "tx_fee_subsidy_total";

/// Max subsided transactions per user
pub const TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY: &str = "tx_fee_subsidy_max_txs_per_user";

/// Karma reward amount in KCents
pub const KARMA_REWARD_AMOUNT: &str = "karma_reward_amount";

/// Number of users to get rewarded each period
pub const KARMA_REWARD_TOP_N_USERS_KEY: &str = "karma_reward_top_n_users";

/// Max number of karma rewards
pub const KARAM_REWARDS_ALLOCATION_KEY: &str = "karma_rewards_allocation";

/// Treasury account id
pub const TREASURY_ACCOUNT_ID_KEY: &str = "treasury_account_id";

/// Treasury pre-minted amount in KCents
pub const TREASURY_PREMINT_COINS_AMOUNT_KEY: &str = "treasury_premint_coins";

/// This service handles the kc blockchain genesis configuration
/// It provides default values for development, and merges in values from
/// a genesis config file when applicable
#[derive(Default)]
pub struct GenesisConfigService {
    config: Config,
    config_file: Option<String>
}

#[async_trait::async_trait]
impl Actor for GenesisConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {

        info!("GenesisConfigService config...");

        let builder = Config::builder();
        // Set defaults and merge genesis config file to overwrite
        let config = builder
            .set_default(NET_ID_KEY, 1)
            .unwrap()
            .set_default(DEF_TX_FEE_KEY, 100)
            .unwrap()

            .set_default(SIGNUP_REWARD_PHASE1_KEY, 10*(10^6))
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE1_ALLOCATION_KEY, 40*(10^6))
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE2_KEY, 10^6)
            .unwrap()
            .set_default(SIGNUP_REWARD_PHASE2_ALLOCATION_KEY, 100*(10^6))
            .unwrap()

            .set_default(REFERRAL_REWARD_PHASE1_KEY, 10*(10^6))
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE1_ALLOCATION_KEY, 40*(10^6))
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE2_KEY, 10^6)
            .unwrap()
            .set_default(REFERRAL_REWARD_PHASE2_ALLOCATION_KEY, 100*(10^6))
            .unwrap()

            .set_default(KARMA_REWARD_AMOUNT, 10*(10^6))
            .unwrap()
            .set_default(KARMA_REWARD_TOP_N_USERS_KEY, 1_000)
            .unwrap()
            .set_default(KARAM_REWARDS_ALLOCATION_KEY, 250*(10^6))
            .unwrap()

            .set_default(TX_FEE_SUBSIDY_TOTAL_KEY, 250*(10^6))
            .unwrap()
            .set_default(TX_FEE_SUBSIDY_MAX_TXS_PER_USER_KEY, 10)
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
            self.config.merge(config::File::with_name(config_file)).unwrap();
        }

        self.config = config;

        Ok(())
    }
}

impl Service for GenesisConfigService {}

// helpers
impl GenesisConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetValue(key)).await?;
        Ok(res)
    }

    /// helper
    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    /// helper
    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = GenesisConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }

    pub async fn set(key: String, value: String) -> Result<()> {
        let config = GenesisConfigService::from_registry().await?;
        config.call(SetValue { key, value }).await?
    }

    /// helper
    pub async fn set_bool(key: String, value: bool) -> Result<()> {
        let config = GenesisConfigService::from_registry().await?;
        config.call(SetBool { key, value }).await?
    }

    /// helper
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

        debug!(
            "Merging content of config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
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
