use haematite_models::meta::permissions::{Path, Tree};
use haematite_models::meta::user::User;

pub struct UserStore {}

impl UserStore {
    pub fn access_token(&self, _token: &str) -> Option<User> {
        Some(User {
            name: "beryllia".to_string(),
            permissions: Tree::from(vec![Path::from("*")]),
        })
    }
}
