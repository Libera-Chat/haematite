use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::Serialize;

use super::ban::Ban;
use super::channel::Channel;
use super::server::Server;
use super::user::User;

#[derive(Default, Serialize)]
pub struct Network {
    pub me: String,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub servers: HashMap<String, Server>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,
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
