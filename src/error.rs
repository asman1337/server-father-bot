use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Telegram error: {0}")]
    Telegram(#[from] teloxide::RequestError),
    
    #[error("Environment error: {0}")]
    Environment(String),
    
    #[error("Server check error: {0}")]
    ServerCheck(String),
}

pub type Result<T> = std::result::Result<T, BotError>; 