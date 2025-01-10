use std::time::Duration;
use teloxide::prelude::*;
use crate::error::Result;
use crate::config::Config;
use crate::db::Database;
use crate::services::server::ServerService;
use crate::db::entities::server::Model as ServerModel;
use crate::monitor;
use crate::monitor::tasks;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerFatherBot {
    bot: Bot,
    db: Database,
    config: Config,
    server_service: ServerService,
}

impl ServerFatherBot {
    pub async fn new(bot: Bot, db: Database, config: Config) -> Self {
        let server_service = ServerService::new(db.connection.clone());
        
        Self { 
            bot,
            db,
            config,
            server_service,
        }
    }

    pub fn server_service(&self) -> &ServerService {
        &self.server_service
    }

    pub async fn check_server_status(&self, server: &ServerModel) -> Result<bool> {
        monitor::check_server(
            &server.host,
            server.port as u16,
            Duration::from_secs(5),
        ).await
    }

    pub async fn notify_status_change(&self, server: &ServerModel, is_up: bool, chat_id: ChatId) -> Result<()> {
        let message = if is_up {
            format!("✅ Server '{}' is back online!", server.name)
        } else {
            format!("🚨 Server '{}' is down!", server.name)
        };

        self.bot.send_message(chat_id, message).await?;
        Ok(())
    }

    pub async fn start_monitoring(&self, chat_id: ChatId) -> Result<()> {
        tasks::start_monitoring_task(Arc::new(self.clone()), chat_id).await
    }
} 