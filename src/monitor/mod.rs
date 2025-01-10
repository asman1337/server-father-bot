use std::time::Duration;
use tokio::net::TcpStream;
use crate::error::Result;

pub async fn check_server(host: &str, port: u16, timeout: Duration) -> Result<bool> {
    match TcpStream::connect((host, port)).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
} 