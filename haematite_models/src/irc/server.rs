use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub users: HashSet<String>,
}
