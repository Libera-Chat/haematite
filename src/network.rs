use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::ban::Ban;
use crate::channel::Channel;
use crate::server::Server;

#[derive(Default)]
pub struct Network {
    pub me: [u8; 3],
    pub servers: HashMap<[u8; 3], Server>,
    pub channels: HashMap<Vec<u8>, Channel>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,
}

impl Network {
    pub fn new(me: Server) -> Self {
        let sid: [u8; 3] = me.sid.clone().as_bytes().try_into().unwrap();
        let mut network = Network {
            me: sid.clone(),
            ..Self::default()
        };
        network.servers.insert(sid, me);
        network
    }
}
