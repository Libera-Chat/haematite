use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::ban::Ban;
use crate::channel::Channel;
use crate::server::Server;

#[derive(Default)]
pub struct Network {
    pub me: Server,
    pub servers: HashMap<[u8; 3], Server>,
    channels: HashMap<String, Channel>,
    pub bans: HashMap<char, LinkedHashMap<String, Ban>>,
}

impl Network {
    pub fn new(me: Server) -> Self {
        Network {
            me,
            ..Self::default()
        }
    }

    pub fn add_channel(&mut self, name: String, channel: Channel) -> bool {
        self.channels.insert(name, channel).is_none()
    }

    pub fn get_channel_mut(&mut self, name: &str) -> &mut Channel {
        self.channels.get_mut(name).unwrap()
    }
}
