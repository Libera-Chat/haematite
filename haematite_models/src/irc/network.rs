use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Serializer};

use super::ban::Ban;
use super::channel::{Channel, Diff as ChannelDiff};
use super::error::Error;
use super::server::{Diff as ServerDiff, Server};
use super::user::{Diff as UserDiff, User};

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

    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(String, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::Ban(mask, action) => {
                let path = format!("bans/{}", mask);
                let value = match action {
                    Action::Add(ban) => {
                        let value = ban.serialize(ser)?;
                        self.bans.insert(mask, ban);
                        value
                    }
                    Action::Remove => {
                        self.bans.remove(&mask);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }

            Diff::ExternalServer(name, action) => {
                let path = format!("servers/{}", name);
                let value = match action {
                    Action::Add(server) => {
                        let value = server.serialize(ser)?;
                        self.servers.insert(name, server);
                        value
                    }
                    Action::Remove => {
                        self.servers.remove(&name);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }
            Diff::InternalServer(name, diff) => {
                let (path, value) = self
                    .servers
                    .get_mut(&name)
                    .ok_or(Error::UnknownServer)?
                    .update(diff, ser)?;
                (format!("servers/{}/{}", name, path), value)
            }

            Diff::ExternalUser(uid, action) => {
                let path = format!("users/{}", uid);
                let value = match action {
                    Action::Add(user) => {
                        let value = user.serialize(ser)?;
                        self.users.insert(uid, user);
                        value
                    }
                    Action::Remove => {
                        self.users.remove(&uid);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }
            Diff::InternalUser(uid, diff) => {
                let (path, value) = self
                    .users
                    .get_mut(&uid)
                    .ok_or(Error::UnknownUser)?
                    .update(diff, ser)?;
                (format!("users/{}/{}", uid, path), value)
            }

            Diff::ExternalChannel(name, action) => {
                let path = format!("channels/{}", name);
                let value = match action {
                    Action::Add(channel) => {
                        let value = channel.serialize(ser)?;
                        self.channels.insert(name, channel);
                        value
                    }
                    Action::Remove => {
                        self.channels.remove(&name);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }
            Diff::InternalChannel(name, diff) => {
                let (path, value) = self
                    .channels
                    .get_mut(&name)
                    .ok_or(Error::UnknownChannel)?
                    .update(diff, ser)?;
                (format!("users/{}/{}", name, path), value)
            }
        })
    }

    /// Find a user by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a user's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of users
    pub fn get_user(&self, id: &str) -> Result<&User, Error> {
        self.users.get(id).ok_or(Error::UnknownUser)
    }

    /// Find a user by its ID, in mutable form.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a user's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of users.
    pub fn get_user_mut(&mut self, id: &str) -> Result<&mut User, Error> {
        self.users.get_mut(id).ok_or(Error::UnknownUser)
    }

    /// Find a channel by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a channel's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of channels.
    pub fn _get_channel(&self, id: &str) -> Result<&Channel, Error> {
        self.channels.get(id).ok_or(Error::UnknownChannel)
    }

    /// Find a channel by its ID, in mutable form.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a channel's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of channels.
    pub fn get_channel_mut(&mut self, id: &str) -> Result<&mut Channel, Error> {
        self.channels.get_mut(id).ok_or(Error::UnknownChannel)
    }

    /// Find a server by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a server's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of servers.
    pub fn get_server(&self, id: &str) -> Result<&Server, Error> {
        self.servers.get(id).ok_or(Error::UnknownServer)
    }

    /// Find a server by its ID, in mutable form.
    ///
    /// # Arguments
    ///
    /// * `id` - A non-decoded binary sequence representing a server's ID,
    /// which is usually data straight from a socket.
    ///
    /// # Errors
    ///
    /// Errors if `id` isn't found in our collection of servers.
    pub fn _get_server_mut(&mut self, id: &str) -> Result<&mut Server, Error> {
        self.servers.get_mut(id).ok_or(Error::UnknownServer)
    }
}
