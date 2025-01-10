use std::env;
use crate::error::{Result, BotError};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub check_interval: u64,  // in seconds
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| BotError::Environment("DATABASE_URL not set".into()))?;
            
        let check_interval = env::var("CHECK_INTERVAL")
            .unwrap_or_else(|_| "300".into())  // default 5 minutes
            .parse()
            .map_err(|_| BotError::Environment("Invalid CHECK_INTERVAL".into()))?;

        Ok(Config {
            database_url,
            check_interval,
        })
    }
} 