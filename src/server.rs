use crate::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct Server {
    pub sid: String,
    pub name: String,
    pub description: String,
    pub users: HashMap<[u8; 9], User>,
}

impl Server {
    pub fn new(sid: String, name: String, description: String) -> Self {
        Self {
            sid,
            name,
            description,
            ..Self::default()
        }
    }
}
