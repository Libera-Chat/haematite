use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub description: String,
}

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
    IoError(std::io::Error),
    YamlParseError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
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
            .map_err(|e| Error::YamlParseError(e.to_string()))
    }
}
