use std::collections::{HashMap, HashSet};

use linked_hash_map::LinkedHashMap;

use crate::ban::Ban;
use crate::channel::{Channel, Membership};
use crate::server::Server;
use crate::user::User;

#[derive(Default)]
pub struct Network {
    pub me: Vec<u8>,
    pub users: HashMap<Vec<u8>, User>,
    pub channels: HashMap<Vec<u8>, Channel>,
    pub servers: HashMap<Vec<u8>, Server>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,

    pub user_channels: HashMap<Vec<u8>, HashMap<Vec<u8>, Membership>>,
    pub channel_users: HashMap<Vec<u8>, HashSet<Vec<u8>>>,

    pub user_server: HashMap<Vec<u8>, Vec<u8>>,
    pub server_users: HashMap<Vec<u8>, HashSet<Vec<u8>>>,
}

impl Network {
    pub fn new(me: Server) -> Self {
        let sid = me.sid.as_bytes().to_vec();
        let mut network = Network {
            me: sid.clone(),
            ..Self::default()
        };
        network.servers.insert(sid, me);
        network
    }
}
