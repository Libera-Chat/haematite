use crate::server::Server;
use std::collections::HashMap;

#[derive(Default)]
pub struct Network {
    servers: HashMap<String, Server>,
}

impl Network {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.insert(server.sid.clone(), server);
    }
}
