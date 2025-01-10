mod bot;
mod commands;
mod config;
mod db;
mod error;
mod monitor;

use anyhow::Result;
use teloxide::prelude::*;
use tracing::info;
use std::sync::Arc;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Server Father Bot...");

    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize configuration
    let config = config::Config::from_env()?;
    
    // Initialize database
    let database = db::Database::new(&config.database_url).await?;
    
    // Get bot token from environment
    let bot = Bot::from_env();
    
    // Initialize bot instance
    let bot_instance = Arc::new(ServerFatherBot::new(
        bot.clone(),
        database,
        config,
    ).await);

    // Start command handler with dialogue support
    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<commands::State>, commands::State>()
        .dispatch_with_handler(commands::schema());

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<commands::State>::new(), bot_instance])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
