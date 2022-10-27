mod user_store;

use crate::user_store::UserStore;
use sqlx::{Database as SqlxDatabase, Pool};

pub struct Database<D: SqlxDatabase> {
    _connection: Pool<D>,
    pub user_store: UserStore,
}

impl<D: SqlxDatabase> Database<D> {
    pub fn from(_connection: Pool<D>) -> Self {
        Self {
            _connection,
            user_store: UserStore {},
        }
    }
}
