use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde::Serialize;

use super::ban::Ban;
use super::channel::Channel;
use super::server::Server;
use super::user::User;

#[derive(Default, Serialize)]
#[must_use]
pub struct Network {
    pub me: Server,
    pub users: HashMap<String, User>,
    pub channels: HashMap<String, Channel>,
    pub servers: HashMap<String, Server>,
    pub bans: LinkedHashMap<String, Ban>,
}
