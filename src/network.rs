use crate::server::Server;
use std::collections::HashMap;

#[derive(Default)]
pub struct Network<'a> {
    servers: HashMap<&'a str, &'a Server<'a>>,
}

impl<'a> Network<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_server(&mut self, server: &'a Server) {
        self.servers.insert(server.sid, server);
        self.servers.insert(server.name, server);
    }
}
