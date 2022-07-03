use crate::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct Server {
    pub sid: String,
    pub name: String,
    pub description: String,
    pub users: HashMap<String, User>,
}
