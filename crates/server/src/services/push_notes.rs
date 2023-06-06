use anyhow::{Error, Result};
use base::server_config_service::ServerConfigService;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct PaymentTxPushNotesData {
    pub(crate) tx_id: String,
    pub(crate) amount: String,
    // base64 accountId
    pub(crate) to_id: String,
    pub(crate) char_id: u32,
    pub(crate) emoji: String,
}

/// Sends a push notification to the user regarding new tx sent to him going onchain
pub(crate) async fn send_tx_push_note(params: PaymentTxPushNotesData) -> Result<()> {
    let token = ServerConfigService::get("cloud_functions.token".into())
        .await?
        .unwrap();

    let endpoint = ServerConfigService::get("cloud_functions.endpoint".into())
        .await?
        .unwrap();

    let mut map = HashMap::new();
    map.insert("toId", params.to_id);
    map.insert("amount", params.amount);
    map.insert("txId", params.tx_id);
    map.insert("charTrait", params.char_id.to_string());
    map.insert("emoji", params.emoji);

    let client = reqwest::Client::new();

    info!(
        "Sending request to {}, with auth token {} and params {:?}",
        endpoint, token, map,
    );

    match client
        .post(format!("https://{}", endpoint))
        .header(reqwest::header::AUTHORIZATION, format!("bearer {}", token))
        .json(&map)
        .send()
        .await
    {
        Ok(_) => {
            info!("Push note request sent");
            Ok(())
        }
        Err(e) => {
            error!("Failed to send push note request {}", e);
            Err(Error::msg(format!(
                "Failed to send push note request {}",
                e
            )))
        }
    }
}
