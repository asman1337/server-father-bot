use crate::config::Config;
use crate::db::entities::server::Model as ServerModel;
use crate::error::Result;
use crate::monitor;
use crate::monitor::tasks;
use crate::services::group::GroupService;
use crate::services::server::ServerService;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ServerFatherBot {
    pub bot: Bot,
    pub config: Config,
    server_service: ServerService,
    group_service: GroupService,
    chat_ids: Arc<Mutex<HashMap<i64, bool>>>,
}

impl ServerFatherBot {
    pub fn new(
        bot: Bot,
        config: Config,
        server_service: ServerService,
        group_service: GroupService,
    ) -> Self {
        Self {
            bot,
            config,
            server_service,
            group_service,
            chat_ids: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn server_service(&self) -> &ServerService {
        &self.server_service
    }

    pub fn group_service(&self) -> &GroupService {
        &self.group_service
    }

    pub fn bot(&self) -> &Bot {
        &self.bot
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn check_server_status(&self, server: &ServerModel) -> Result<bool> {
        monitor::check_server(&server.host, server.port as u16, Duration::from_secs(5)).await
    }

    pub async fn notify_status_change(
        &self,
        server: &ServerModel,
        is_up: bool,
        chat_id: ChatId,
    ) -> Result<()> {
        let message = if is_up {
            format!("✅ Server '{}' is back online!", server.name)
        } else {
            format!("🚨 Server '{}' is down!", server.name)
        };

        self.bot.send_message(chat_id, message).await?;
        Ok(())
    }

    pub async fn start_monitoring(&self, chat_id: ChatId) -> Result<()> {
        tokio::spawn(tasks::monitor_servers(Arc::new(self.clone()), chat_id.0));
        Ok(())
    }
}
