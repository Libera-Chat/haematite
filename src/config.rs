use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub server_name: String,
    pub sid: String,
    pub server_description: String,

    pub uplink_remote_address: String,
    pub uplink_remote_port: u16,
    pub uplink_password: String,
}

#[derive(Debug)]
pub enum HMConfigError {
    InvalidSid,
    InvalidServerName,
    IoError(std::io::Error),
    YamlParseError(String),
}

impl From<std::io::Error> for HMConfigError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl std::fmt::Display for HMConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for HMConfigError {}

impl Config {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self, HMConfigError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let deserialized_config = match serde_yaml::from_reader::<BufReader<File>, Config>(reader) {
            Ok(it) => it,
            Err(err) => {
                return Err(HMConfigError::YamlParseError(err.to_string()));
            }
        };
        deserialized_config.validate()
    }

    fn validate(self) -> Result<Self, HMConfigError> {
        let sid = self.sid.as_bytes();

        if sid.len() != 3
            || !sid[0].is_ascii_digit()
            || !(sid[1].is_ascii_uppercase() || sid[1].is_ascii_digit())
            || !(sid[2].is_ascii_uppercase() || sid[2].is_ascii_digit())
        {
            return Err(HMConfigError::InvalidSid);
        }

        if !self
            .server_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.')
        {
            return Err(HMConfigError::InvalidServerName);
        }

        Ok(self)
    }
}
