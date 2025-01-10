pub mod tasks;

use crate::error::Result;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn check_server(host: &str, port: u16, timeout_duration: Duration) -> Result<bool> {
    let addr = format!("{}:{}", host, port);

    match timeout(timeout_duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => Ok(true),
        Ok(Err(_)) => Ok(false),
        Err(_) => Ok(false), // Timeout occurred
    }
}
