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
    pub fn add_user(&mut self, uid: String, user: User) -> bool {
        self.users.insert(uid, user).is_none()
    }

    pub fn get_user_mut(&mut self, uid: &str) -> &mut User {
        self.users.get_mut(uid).unwrap()
    }
}
