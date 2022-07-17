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
    Io(std::io::Error),
    Parse(Box<dyn std::error::Error>),
    // The following two might also have uses for fields later.
    InvalidName,
    InvalidId,
    // Maybe add errors for Tls validation failure?
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {}", e),
            Error::Parse(e) => write!(f, "parse error: {}", e),
            Error::InvalidName => write!(f, "invalid server name"),
            Error::InvalidId => write!(f, "invalid server id"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Parse(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
