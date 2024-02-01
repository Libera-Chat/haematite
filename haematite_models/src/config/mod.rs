use std::path::PathBuf;

use serde::Deserialize;

use crate::irc::server::Server;
#[derive(Debug, Deserialize)]
pub struct Uplink {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Mtls {
    pub ca: PathBuf,
    pub crt: PathBuf,
    pub key: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Amqp {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub uplink: Uplink,
    pub mtls: Mtls,
    pub amqp: Amqp,
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
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Parse(e) => write!(f, "parse error: {e}"),
            Self::InvalidName => write!(f, "invalid server name"),
            Self::InvalidId => write!(f, "invalid server id"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Parse(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
