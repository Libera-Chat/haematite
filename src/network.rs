use crate::channel::Channel;
use crate::server::Server;
use std::collections::HashMap;

#[derive(Default)]
pub struct Network {
    servers: HashMap<String, Server>,
    channels: HashMap<String, Channel>,
}

impl Network {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_server(&mut self, server: Server) -> bool {
        self.servers.insert(server.sid.clone(), server).is_none()
    }

    pub fn del_server(&mut self, sid: &str) -> bool {
        self.servers.remove(sid).is_some()
    }

    pub fn get_server_mut(&mut self, sid: &str) -> &mut Server {
        self.servers.get_mut(sid).unwrap()
    }

    pub fn add_channel(&mut self, name: String, channel: Channel) -> bool {
        self.channels.insert(name, channel).is_none()
    }
}
