use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::ban::Ban;
use crate::channel::Channel;
use crate::server::Server;

#[derive(Default)]
pub struct Network {
    pub me: Server,
    pub servers: HashMap<[u8; 3], Server>,
    pub channels: HashMap<Vec<u8>, Channel>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,
}

impl Network {
    pub fn new(me: Server) -> Self {
        Network {
            me,
            ..Self::default()
        }
    }
}
