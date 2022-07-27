use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub users: HashSet<String>,
}

impl Server {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            ..Self::default()
        }
    }
}
