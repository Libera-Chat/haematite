#![allow(clippy::too_many_lines)]
use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Serializer};

use super::ban::Ban;
use super::channel::{Channel, Diff as ChannelDiff};
use super::error::Error;
use super::server::{Diff as ServerDiff, Server};
use super::user::{Diff as UserDiff, User};
use crate::meta::permissions::Path;

#[derive(Default, Serialize)]
pub struct Network {
    pub me: String,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub servers: HashMap<String, Server>,
    pub bans: LinkedHashMap<String, Ban>,
}

pub enum Action<T> {
    Add(T),
    Remove,
}

pub enum Diff {
    Ban(String, Action<Ban>),

    InternalServer(String, ServerDiff),
    ExternalServer(String, Action<Server>),

    InternalUser(String, UserDiff),
    ExternalUser(String, Action<User>),

    InternalChannel(String, ChannelDiff),
    ExternalChannel(String, Action<Channel>),
}

impl Network {
    pub fn new(me: Server) -> Self {
        let sid = me.id.clone();
        let mut network = Network {
            me: sid.clone(),
            ..Self::default()
        };
        network.servers.insert(sid, me);
        network
    }

    /// # Errors
    ///
    /// Will return `Err` if the presented diff is not applicable to the
    /// current network state, or if the result data cannot be serialized.
    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(Path, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::Ban(mask, action) => {
                let value = match action {
                    Action::Add(ban) => {
                        let value = ban.serialize(ser)?;
                        self.bans.insert(mask.clone(), ban);
                        value
                    }
                    Action::Remove => {
                        self.bans.remove(&mask);
                        ser.serialize_none()?
                    }
                };
                (
                    Path::InternalVertex("bans".to_string(), Box::new(Path::ExternalVertex(mask))),
                    value,
                )
            }

            Diff::ExternalServer(name, action) => {
                let value = match action {
                    Action::Add(server) => {
                        let value = server.serialize(ser)?;
                        self.servers.insert(name.clone(), server);
                        value
                    }
                    Action::Remove => {
                        self.servers.remove(&name);
                        ser.serialize_none()?
                    }
                };
                (
                    Path::InternalVertex(
                        "servers".to_string(),
                        Box::new(Path::ExternalVertex(name)),
                    ),
                    value,
                )
            }
            Diff::InternalServer(name, diff) => {
                let (path, value) = self
                    .servers
                    .get_mut(&name)
                    .ok_or(Error::UnknownServer)?
                    .update(diff, ser)?;
                (
                    Path::InternalVertex(
                        "servers".to_string(),
                        Box::new(Path::InternalVertex(name, Box::new(path))),
                    ),
                    value,
                )
            }

            Diff::ExternalUser(uid, action) => {
                let value = match action {
                    Action::Add(user) => {
                        let value = user.serialize(ser)?;
                        self.users.insert(uid.clone(), user);
                        value
                    }
                    Action::Remove => {
                        self.users.remove(&uid);
                        ser.serialize_none()?
                    }
                };
                (
                    Path::InternalVertex("users".to_string(), Box::new(Path::ExternalVertex(uid))),
                    value,
                )
            }
            Diff::InternalUser(uid, diff) => {
                let (path, value) = self
                    .users
                    .get_mut(&uid)
                    .ok_or(Error::UnknownUser)?
                    .update(diff, ser)?;
                (
                    Path::InternalVertex(
                        "users".to_string(),
                        Box::new(Path::InternalVertex(uid, Box::new(path))),
                    ),
                    value,
                )
            }

            Diff::ExternalChannel(name, action) => {
                let value = match action {
                    Action::Add(channel) => {
                        let value = channel.serialize(ser)?;
                        self.channels.insert(name.clone(), channel);
                        value
                    }
                    Action::Remove => {
                        self.channels.remove(&name);
                        ser.serialize_none()?
                    }
                };
                (
                    Path::InternalVertex(
                        "channels".to_string(),
                        Box::new(Path::ExternalVertex(name)),
                    ),
                    value,
                )
            }
            Diff::InternalChannel(name, diff) => {
                let (path, value) = self
                    .channels
                    .get_mut(&name)
                    .ok_or(Error::UnknownChannel)?
                    .update(diff, ser)?;
                (
                    Path::InternalVertex(
                        "channels".to_string(),
                        Box::new(Path::InternalVertex(name, Box::new(path))),
                    ),
                    value,
                )
            }
        })
    }
}
