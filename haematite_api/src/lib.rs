use std::sync::{Arc, PoisonError, RwLock};

use serde::Serialize;
use serde_json::Error as JsonError;
use tokio::sync::mpsc;

use haematite_models::irc::network::Network;
use haematite_models::meta::permissions::Path;
use haematite_models::meta::user::User;
use haematite_ser::error::Error as SerError;
use haematite_ser::{Allow, Serializer, WrapType};

pub enum Format {
    Terse,
    Pretty,
}

pub struct Api {
    network: Arc<RwLock<Network>>,
    format: Format,
    streams: Vec<(User, mpsc::Sender<(Path, String)>)>,
}

#[derive(Debug)]
pub enum Error {
    Serialize,
    Argument,
    Concurrency,
    Unauthorized,
}

impl From<JsonError> for Error {
    fn from(_error: JsonError) -> Self {
        Self::Serialize
    }
}

impl From<SerError> for Error {
    fn from(_error: SerError) -> Self {
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
        Self {
            network,
            format,
            streams: Vec::new(),
        }
    }

    fn format<T: Serialize>(&self, value: &T) -> Result<String, JsonError> {
        Ok(match self.format {
            Format::Terse => serde_json::to_string(value)?,
            Format::Pretty => serde_json::to_string_pretty(value)?,
        })
    }

    pub async fn read_stream(
        api: Arc<RwLock<Self>>,
        stream: &mut mpsc::Receiver<(Path, WrapType)>,
    ) -> Result<(), Error> {
        let mut dead = Vec::new();

        while let Some((path, mut wrap)) = stream.recv().await {
            let mut api = api.write()?;
            for (i, (user, sender)) in api.streams.iter().enumerate() {
                if let Some(tree) = user.permissions.walk(&path) {
                    if wrap.update_with(tree) == Allow::Yes
                        && sender.try_send((path.clone(), api.format(&wrap)?)).is_err()
                    {
                        dead.push(i);
                    }
                }
            }

            /* if `dead` is `[1, 2]` then after we remove `1`, `2`'s index
             * would have shifted down one, so we iterate in reverse
             */
            for i in dead.drain(..).rev() {
                api.streams.remove(i);
            }
        }
        Ok(())
    }

    pub fn subscribe_stream(&mut self, user: User) -> mpsc::Receiver<(Path, String)> {
        let (tx, rx) = mpsc::channel(1);
        self.streams.push((user, tx));
        rx
    }

    pub fn get_network(&self, user: &User) -> Result<String, Error> {
        let mut network = self.network.read()?.serialize(&mut Serializer {})?;
        network.update_with(&user.permissions);
        Ok(self.format(&network)?)
    }

    pub fn get_user(&self, user: &User, uid: &str) -> Result<String, Error> {
        let network = self.network.read()?;
        let mut network_user = network
            .users
            .get(uid)
            .ok_or(Error::Argument)?
            .serialize(&mut Serializer {})?;

        if let Some(tree) = user
            .permissions
            .walk(&Path::from(&format!("users/{}", uid)))
        {
            network_user.update_with(tree);
            Ok(self.format(&network_user)?)
        } else {
            Err(Error::Unauthorized)
        }
    }
}
