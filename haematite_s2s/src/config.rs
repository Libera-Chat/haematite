use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::path::Path;

use haematite_models::server::Server;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Uplink {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub uplink: Uplink,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidYaml(String),
    InvalidData(String),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        from_reader::<BufReader<File>, Config>(reader)
            .map_err(|e| Error::InvalidYaml(e.to_string()))
    }
}
