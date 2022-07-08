use crate::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct Server {
    pub sid: String,
    pub name: String,
    pub description: String,
    pub users: HashMap<Vec<u8>, User>,
}

impl Server {
    pub fn add_user(&mut self, uid: Vec<u8>, user: User) -> bool {
        self.users.insert(uid, user).is_none()
    }

    pub fn del_user(&mut self, uid: &[u8]) -> bool {
        self.users.remove(uid).is_some()
    }

    pub fn get_user_mut(&mut self, uid: &[u8]) -> &mut User {
        self.users.get_mut(uid).unwrap()
    }
}
