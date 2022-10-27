use haematite_models::irc::network::Network;
use serde_json::{Error as JsonError, Value};

pub enum Format {
    Terse,
    Pretty,
}

pub struct Api {
    format: Format,
}

#[derive(Debug)]
pub enum Error {
    Serialize,
    Argument,
}

impl From<JsonError> for Error {
    fn from(_error: JsonError) -> Self {
        Self::Serialize
    }
}

impl Api {
    pub fn new(format: Format) -> Self {
        Self { format }
    }

    fn format(&self, value: Value) -> Result<String, JsonError> {
        Ok(match self.format {
            Format::Terse => serde_json::to_string(&value)?,
            Format::Pretty => serde_json::to_string_pretty(&value)?,
        })
    }

    pub fn get_network(&self, network: &Network) -> Result<String, Error> {
        let value = serde_json::to_value(network)?;
        Ok(self.format(value)?)
    }

    pub fn get_user(&self, network: &Network, uid: &str) -> Result<String, Error> {
        let user = network.users.get(uid).ok_or(Error::Argument)?;
        let value = serde_json::to_value(user)?;
        Ok(self.format(value)?)
    }
}
