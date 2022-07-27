mod permissions;

use std::collections::HashSet;

use haematite_models::network::Network;
use serde_json::{Error as JsonError, Map, Value};

use crate::permissions::{MergeError, Tree, Vertex};

fn prune(paths: &Tree, value: Value) -> Option<Value> {
    let all = paths.get("*");
    let empty = Tree::new();

    if let Value::Object(map_old) = value {
        let mut map_new = Map::new();

        for (key, value) in map_old.into_iter() {
            if let Some(paths) = paths.get(&key).or(all) {
                let paths = match paths {
                    // this branch of permissions expects more tree
                    Vertex::Internal(paths) => paths,
                    // this branch of permissions expects the end of a tree
                    Vertex::External => &empty,
                };

                if let Some(value) = prune(paths, value) {
                    map_new.insert(key, value);
                }
            }
        }

        if map_new.is_empty() {
            None
        } else {
            Some(Value::Object(map_new))
        }
    } else {
        Some(value)
    }
}

pub enum Format {
    Terse,
    Pretty,
}

pub struct Api {
    format: Format,
}

#[derive(Debug)]
pub enum Error {
    Serialize,
    Argument,
}

impl From<JsonError> for Error {
    fn from(_error: JsonError) -> Self {
        Self::Serialize
    }
}

impl From<MergeError> for Error {
    fn from(_error: MergeError) -> Self {
        Self::Argument
    }
}

impl Api {
    pub fn new(format: Format) -> Self {
        Self { format }
    }

    fn format(&self, value: Value) -> Result<String, JsonError> {
        Ok(match self.format {
            Format::Terse => serde_json::to_string(&value)?,
            Format::Pretty => serde_json::to_string_pretty(&value)?,
        })
    }

    pub fn get_network(&self, network: &Network) -> Result<String, Error> {
        let mut paths = Tree::from_paths(&HashSet::from(["users/*/nick"]));
        paths.add_path("users/00AAAAAAG/host").unwrap();

        let value = prune(&paths, serde_json::to_value(network)?).ok_or(Error::Argument)?;

        Ok(self.format(value)?)
    }

    pub fn get_user(&self, network: &Network, uid: &str) -> Result<String, Error> {
        let paths = Tree::from_paths(&HashSet::from(["users/00AAAAAAG/nick"]));
        let user = network.users.get(uid).ok_or(Error::Argument)?;

        let relevant_paths = paths
            .find_tree(format!("users/{}", uid).as_str())
            .ok_or(Error::Argument)?;
        let value = prune(relevant_paths, serde_json::to_value(user)?).ok_or(Error::Argument)?;

        Ok(self.format(value)?)
    }
}
