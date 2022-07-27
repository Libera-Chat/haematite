mod permissions;

use std::collections::HashSet;

use haematite_models::network::Network;
use serde_json::{Map, Value};

use crate::permissions::{Tree, Vertex};

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

pub struct Api {}

#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Api {
    pub fn get_network(network: &Network) -> Result<String, serde_json::Error> {
        let mut paths = Tree::from_paths(&HashSet::from(["users/*/nick"]));
        paths.add_path("users/00AAAAAAG/host").unwrap();

        if let Some(value) = prune(&paths, serde_json::to_value(network)?) {
            Ok(value.to_string())
        } else {
            Ok("{}".to_string())
        }
    }
}
