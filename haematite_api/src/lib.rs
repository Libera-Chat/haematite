use std::sync::{Arc, PoisonError, RwLock};

use haematite_models::irc::network::Network;
use haematite_models::meta::user::User;
use serde::Serialize;
use serde_json::Error as JsonError;

pub enum Format {
    Terse,
    Pretty,
}

pub struct Api {
    network: Arc<RwLock<Network>>,
    format: Format,
}

#[derive(Debug)]
pub enum Error {
    Serialize,
    Argument,
    Concurrency,
}

impl From<JsonError> for Error {
    fn from(_error: JsonError) -> Self {
        Self::Serialize
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Self {
        Self::Concurrency
    }
}

impl Api {
    pub fn new(network: Arc<RwLock<Network>>, format: Format) -> Self {
        Self { network, format }
    }

    fn format<T: Serialize>(&self, value: &T) -> Result<String, JsonError> {
        Ok(match self.format {
            Format::Terse => serde_json::to_string(value)?,
            Format::Pretty => serde_json::to_string_pretty(value)?,
        })
    }

    pub fn get_network(&self, _user: &User) -> Result<String, Error> {
        let network = self.network.read()?;
        Ok(self.format(&*network)?)
    }

    pub fn get_user(&self, _user: &User, uid: &str) -> Result<String, Error> {
        let network = self.network.read()?;
        let user = network.users.get(uid).ok_or(Error::Argument)?;
        Ok(self.format(user)?)
    }
}
