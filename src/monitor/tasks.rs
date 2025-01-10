use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error};
use crate::bot::ServerFatherBot;
use crate::error::Result;
use teloxide::types::ChatId;

pub async fn start_monitoring_task(bot: Arc<ServerFatherBot>, chat_id: ChatId) -> Result<()> {
    let interval_secs = bot.config.check_interval;
    info!("Starting monitoring task with interval: {}s", interval_secs);

    let mut interval = interval(Duration::from_secs(interval_secs));

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            if let Err(e) = check_all_servers(&bot, chat_id).await {
                error!("Error checking servers: {}", e);
            }
        }
    });

    Ok(())
}

async fn check_all_servers(bot: &ServerFatherBot, chat_id: ChatId) -> Result<()> {
    let servers = bot.server_service().list_servers().await?;

    for server in servers {
        let is_up = bot.check_server_status(&server).await?;
        
        // Get previous status
        let prev_status = server.is_active;
        
        // If status changed, update database and notify
        if is_up != prev_status {
            bot.server_service()
                .update_server_status(server.id, is_up)
                .await?;
            
            bot.notify_status_change(&server, is_up, chat_id).await?;
        }
    }

    Ok(())
} 