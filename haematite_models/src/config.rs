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

#[derive(Debug)]
pub enum Error {
    InvalidData(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error {}
