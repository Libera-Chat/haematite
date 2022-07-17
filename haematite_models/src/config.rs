use crate::server::Server;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Uplink {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub ca: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Tls {
    pub crt: PathBuf,
    pub key: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub uplink: Uplink,
    pub tls: Tls,
}

// If thiserror gets added, use that.
#[derive(Debug)]
pub enum Error {
    InvalidName,
    InvalidId,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidName => write!(f, "invalid server name"),
            Error::InvalidId => write!(f, "invalid server id"),
        }
    }
}

impl std::error::Error for Error {}
