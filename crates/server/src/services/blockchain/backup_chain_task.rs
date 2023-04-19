use crate::services::blockchain::backup_chain_service::BackupChainService;
use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::blockchain::get_all_users::GetAllUsers;
use crate::services::blockchain::stats::GetStats;
use anyhow::Result;
use base::karma_coin::karma_coin_api::{GetAllUsersRequest, GetBlockchainDataRequest};
use base::karma_coin::karma_coin_core_types::{BlockchainStats, User};
use chrono::prelude::*;
use std::fs::File;
use std::io::Write;
use xactor::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct BackupData {
    time: String,
    stats: BlockchainStats,
    users: Vec<User>,
}

impl BackupChainService {
    pub(crate) async fn backup_chain(&self) -> Result<()> {
        info!("processing backup chain task...");

        let service = BlockChainService::from_registry().await?;

        info!("getting users...");
        let users: Vec<User> = service
            .call(GetAllUsers(GetAllUsersRequest { community_id: 0 }))
            .await??
            .users;

        info!("getting stats...");
        let stats: BlockchainStats = service
            .call(GetStats(GetBlockchainDataRequest {}))
            .await??
            .stats
            .unwrap();

        info!("backing up {} users", users.len());

        let local: DateTime<Local> = Local::now();
        let backup_data = BackupData {
            time: local.format("%c").to_string(),
            stats,
            users,
        };

        let data = serde_json::to_string_pretty(&backup_data)?;

        let file_name = format!("karmachain backup {}.json", local.format("%c"))
            .replace(' ', "_")
            .replace(':', "_");

        let mut file = File::create(format!("./{}", file_name))?;
        file.write_all(data.as_bytes())?;
        info!("backup chain task completed. File: {}", file_name);
        Ok(())
    }
}

#[message(result = "Result<()>")]
pub(crate) struct BackupChain;

#[async_trait::async_trait]
impl Handler<BackupChain> for BackupChainService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: BackupChain) -> Result<()> {
        info!("calling backup chain...");
        self.backup_chain().await
    }
}
