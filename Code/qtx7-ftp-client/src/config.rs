// Configuration management for FTP client

use anyhow::{Result, Context};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub ftp_host: String,
    pub ftp_port: u16,
    pub ftp_user: String,
    pub ftp_pass: String,
    pub entity_name: String,
    pub private_key_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            ftp_host: env::var("FTP_HOST")
                .context("FTP_HOST environment variable not set")?,
            ftp_port: env::var("FTP_PORT")
                .unwrap_or_else(|_| "21".to_string())
                .parse()
                .context("Invalid FTP_PORT")?,
            ftp_user: env::var("FTP_USER")
                .context("FTP_USER environment variable not set")?,
            ftp_pass: env::var("FTP_PASS")
                .context("FTP_PASS environment variable not set")?,
            entity_name: env::var("ENTITY_NAME")
                .unwrap_or_else(|_| "QTX-7.4".to_string()),
            private_key_path: env::var("PRIVATE_KEY_PATH")
                .unwrap_or_else(|_| "./qtx7_private_key.pem".to_string()),
        })
    }
}
