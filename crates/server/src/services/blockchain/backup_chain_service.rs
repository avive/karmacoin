// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::backup_chain_task::BackupChain;
use anyhow::Result;
use base::genesis_config_service::{GenesisConfigService, BACKUP_CHAIN_TASK_PERIOD_MINUTES};
use tokio::spawn;
use tokio_schedule::{every, Job};
use xactor::*;

/// A simple transactions pool service
/// This service is used to store transactions that are not yet included in a block
#[derive(Debug, Clone, Default)]
pub(crate) struct BackupChainService {}

impl Service for BackupChainService {}

#[async_trait::async_trait]
impl Actor for BackupChainService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("started. Registering periodic chain backup task...");

        let task_period_min = GenesisConfigService::get_u64(BACKUP_CHAIN_TASK_PERIOD_MINUTES.into())
            .await?
            .unwrap() as u32;

        let task = every(task_period_min).minutes().perform(|| async {
            let service = BackupChainService::from_registry().await;
            if service.is_err() {
                error!("BlockChainService not available");
                return;
            }

            info!("Starting periodic backup chain processing task...");
            match service.unwrap().call(BackupChain).await {
                Ok(res) => match res {
                    Ok(_) => info!("Backup chain task completed"),
                    Err(e) => error!("Backup chain task processing task error: {}", e),
                },
                Err(e) => error!("Error running backup chain task: {}", e),
            }
        });
        spawn(task);

        //self.backup_chain().await?;

        Ok(())
    }
}
