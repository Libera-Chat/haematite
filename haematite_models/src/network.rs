use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::ban::Ban;
use crate::channel::Channel;
use crate::server::Server;
use crate::user::User;

#[derive(Default)]
pub struct Network {
    pub me: Vec<u8>,
    pub users: HashMap<Vec<u8>, User>,
    pub channels: HashMap<Vec<u8>, Channel>,
    pub servers: HashMap<Vec<u8>, Server>,
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
        let sid = me.id.as_bytes().to_vec();
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
    pub fn get_user(&self, id: &[u8]) -> Result<&User, Error> {
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
    pub fn get_user_mut(&mut self, id: &[u8]) -> Result<&mut User, Error> {
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
    pub fn _get_channel(&self, id: &[u8]) -> Result<&Channel, Error> {
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
    pub fn get_channel_mut(&mut self, id: &[u8]) -> Result<&mut Channel, Error> {
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
    pub fn _get_server(&self, id: &[u8]) -> Result<&Server, Error> {
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
    pub fn _get_server_mut(&mut self, id: &[u8]) -> Result<&mut Server, Error> {
        self.servers.get_mut(id).ok_or(Error::UnknownServer)
    }
}
