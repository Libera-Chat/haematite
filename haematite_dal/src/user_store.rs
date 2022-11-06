use std::sync::Arc;

use haematite_models::meta::permissions::{Path, Tree};
use haematite_models::meta::user::User;
use sqlx::{Database as SqlxDatabase, Pool};

pub struct UserStore<D: SqlxDatabase> {
    _connection: Arc<Pool<D>>,
}

impl<D: SqlxDatabase> UserStore<D> {
    pub fn new(connection: Arc<Pool<D>>) -> Self {
        Self {
            _connection: connection,
        }
    }

    pub async fn access_token(&self, _token: &str) -> Option<User> {
        Some(User {
            name: "beryllia".to_string(),
            permissions: Tree::from(vec![Path::from("*")]),
        })
    }
}
