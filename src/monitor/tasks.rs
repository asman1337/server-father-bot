use crate::bot::ServerFatherBot;
use std::sync::Arc;
use teloxide::{prelude::*, types::ChatId};
use tokio::time::{sleep, Duration};

pub async fn monitor_servers(bot: Arc<ServerFatherBot>, chat_id: i64) {
    let interval_secs = bot.config().check_interval;
    let chat_id = ChatId(chat_id);

    loop {
        let mut current_status = Vec::new();

        let servers = match bot.server_service().list_servers().await {
            Ok(servers) => servers,
            Err(e) => {
                let _ = bot
                    .bot()
                    .send_message(chat_id, format!("❌ Failed to fetch servers: {}", e))
                    .await;
                continue;
            }
        };

        // Check all servers and collect their status
        for server in servers {
            let is_up = bot.check_server_status(&server).await.unwrap_or(false);
            current_status.push((server, is_up));
        }

        // Process status changes and send notifications
        for (server, is_up) in current_status {
            let status_text = if is_up { "🟢 Online" } else { "🔴 Offline" };
            let message = format!(
                "Status Change:\n{} *{}*\n`{}:{}`\nStatus: {}",
                if is_up { "✅" } else { "❌" },
                server.name,
                server.host,
                server.port,
                status_text
            );

            let _ = bot
                .bot()
                .send_message(chat_id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await;
        }

        sleep(Duration::from_secs(interval_secs)).await;
    }
}
