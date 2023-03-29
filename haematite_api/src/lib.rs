use std::sync::{Arc, PoisonError, RwLock};
use std::time::Instant;

use serde::Serialize;
use serde_json::{json, Error as JsonError};
use tokio::sync::mpsc;

use haematite_models::irc::network::{DiffOp, Network};
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
    streams: Vec<(User, mpsc::Sender<String>)>,
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
        stream: &mut mpsc::Receiver<(Path, DiffOp<WrapType>)>,
    ) -> Result<(), Error> {
        let mut dead = Vec::new();

        while let Some((path, diff_op)) = stream.recv().await {
            let path_str = path.to_string();
            let (op, mut value) = match diff_op {
                DiffOp::Add(value) => ("add", value),
                DiffOp::Remove(value) => ("remove", value),
                DiffOp::Replace(value) => ("replace", value),
            };

            let mut api = api.write()?;
            for (i, (user, sender)) in api.streams.iter().enumerate() {
                let now = Instant::now();
                if let Some(tree) = user.permissions.walk(&path) {
                    if value.update_with(tree) == Allow::Yes {
                        let out = json!({
                            "op": op,
                            "path": path_str,
                            "value": value,
                        })
                        .to_string();
                        if sender.try_send(out).is_err() {
                            dead.push(i);
                        }
                    }
                }
                println!(
                    "handled subscriber in {}µs",
                    (now.elapsed().as_nanos() as f64) / 1000.0
                );
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

    pub fn subscribe_stream(&mut self, user: User) -> mpsc::Receiver<String> {
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
