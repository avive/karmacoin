use anyhow::{Error, Result};
use base::server_config_service::ServerConfigService;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct ReferralPushNotesData {
    //   // base64 accountId
    pub(crate) to_id: String,
    pub(crate) amount: String,
}

/// Sends a push notification to a user who got referral reward due to appreciation receiver sign up
pub(crate) async fn send_referral_push_note(params: ReferralPushNotesData) -> Result<()> {
    // todo: put into actor and load this at construction time as these are immutable
    let token = ServerConfigService::get("cloud_functions.token".into())
        .await?
        .unwrap();

    let endpoint = ServerConfigService::get("cloud_functions.referral_push_note.endpoint".into())
        .await?
        .unwrap();

    let mut map = HashMap::new();
    map.insert("toId", params.to_id);
    map.insert("amount", params.amount);

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
