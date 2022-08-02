use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::Serialize;

use super::ban::Ban;
use super::channel::{Channel, Diff as ChannelDiff};
use super::server::Server;
use super::user::{Diff as UserDiff, User};

#[derive(Default, Serialize)]
pub struct Network {
    pub me: String,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub servers: HashMap<String, Server>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,
}

pub enum Action<T> {
    Add(T),
    Remove,
}

pub enum Diff {
    InternalUser(String, UserDiff),
    ExternalUser(String, Action<User>),

    InternalChannel(String, ChannelDiff),
    ExternalChannel(String, Action<Channel>),
}

pub enum Error {
    UnknownBan,
    UnknownChannel,
    UnknownServer,
    UnknownUser,

    OverwrittenBan,
    OverwrittenChannel,
    OverwrittenServer,
    OverwrittenUser,
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

    pub fn update(&mut self, diff: Diff) {
        match diff {
            Diff::ExternalUser(uid, action) => drop(match action {
                Action::Add(user) => self.users.insert(uid, user),
                Action::Remove => self.users.remove(&uid),
            }),
            Diff::ExternalChannel(name, action) => drop(match action {
                Action::Add(channel) => self.channels.insert(name, channel),
                Action::Remove => self.channels.remove(&name),
            }),
            Diff::InternalUser(uid, diff) => self.users.get_mut(&uid).unwrap().update(diff),
            Diff::InternalChannel(name, diff) => self.channels.get_mut(&name).unwrap().update(diff),
        };
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
