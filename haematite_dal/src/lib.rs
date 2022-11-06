mod user_store;

use crate::user_store::UserStore;
use sqlx::{Database as SqlxDatabase, Pool};
use std::sync::Arc;

pub struct Database<D: SqlxDatabase> {
    _connection: Arc<Pool<D>>,
    pub user_store: UserStore<D>,
}

impl<D: SqlxDatabase> Database<D> {
    pub fn from(connection: Pool<D>) -> Self {
        let connection = Arc::new(connection);
        Self {
            user_store: UserStore::new(Arc::clone(&connection)),
            _connection: connection,
        }
    }
}
