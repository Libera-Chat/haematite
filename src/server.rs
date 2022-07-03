use crate::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct Server {
    pub sid: String,
    pub name: String,
    pub description: String,
    pub users: HashMap<String, User>,
}

impl Server {
    pub fn add_user(mut self, user: User) {
        self.users.insert(user.uid.clone(), user);
    }
}
