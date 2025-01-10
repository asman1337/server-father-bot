use teloxide::{RequestError, ApiError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Telegram error: {0}")]
    Telegram(#[from] RequestError),
    
    #[error("Dialogue error: {0}")]
    Dialogue(#[from] teloxide::dispatching::dialogue::InMemStorageError),
    
    #[error("Environment error: {0}")]
    Environment(String),
    
    #[error("Server check error: {0}")]
    ServerCheck(String),
}

impl From<BotError> for RequestError {
    fn from(err: BotError) -> Self {
        RequestError::Api(ApiError::Unknown(err.to_string()))
    }
}

pub type Result<T> = std::result::Result<T, BotError>; 