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

pub async fn check_all_servers(bot: &ServerFatherBot, chat_id: ChatId) -> Result<()> {
    // First check all servers and collect status changes
    let mut status_changes = Vec::new();
    
    // Get all servers grouped by their groups
    let groups = bot.group_service().list_groups().await?;
    let servers = bot.server_service().list_servers().await?;

    // Check ungrouped servers
    let ungrouped_servers: Vec<_> = servers.iter()
        .filter(|s| s.group_id.is_none())
        .collect();

    for server in &ungrouped_servers {
        let is_up = bot.check_server_status(server).await?;
        if is_up != server.is_active {
            status_changes.push((server.clone(), is_up));
        }
    }

    // Check grouped servers
    for group in groups {
        let group_servers = bot.server_service()
            .list_servers_by_group(group.id)
            .await?;

        let mut group_changes = Vec::new();
        
        for server in group_servers {
            let is_up = bot.check_server_status(&server).await?;
            if is_up != server.is_active {
                group_changes.push((server.clone(), is_up));
            }
        }

        // If multiple servers in a group changed status, send a group notification
        if group_changes.len() > 1 {
            let mut message = format!("🔄 Status changes in group '{}':\n", group.name);
            for (server, is_up) in &group_changes {
                let status = if *is_up { "online" } else { "offline" };
                message.push_str(&format!("\n{} is now {}", server.name, status));
            }
            bot.bot.send_message(chat_id, message).await?;
        } else {
            // Add single server changes to the main list
            status_changes.extend(group_changes);
        }
    }

    // Update database and send individual notifications for ungrouped changes
    for (server, is_up) in status_changes {
        bot.server_service()
            .update_server_status(server.id, is_up)
            .await?;
        
        bot.notify_status_change(&server, is_up, chat_id).await?;
    }

    Ok(())
} 