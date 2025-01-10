mod bot;
mod commands;
mod config;
mod db;
mod error;
mod monitor;
mod services;

use crate::bot::ServerFatherBot;
use crate::commands::State;
use crate::config::Config;
use crate::db::Database;
use crate::error::Result;
use crate::services::{group::GroupService, server::ServerService};
use std::sync::Arc;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Server Father bot...");

    dotenvy::dotenv().ok();

    let bot = Bot::from_env();
    let config = Config::from_env()?;
    let database = Database::new(&config.database_url).await?;

    let server_service = ServerService::new(database.connection.clone());
    let group_service = GroupService::new(database.connection.clone());

    let bot_instance = Arc::new(ServerFatherBot::new(
        bot.clone(),
        config,
        server_service,
        group_service,
    ));

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .chain(commands::schema());

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), bot_instance])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
