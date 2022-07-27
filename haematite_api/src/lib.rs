use std::collections::{HashMap, HashSet};

use haematite_models::network::Network;
use serde_json::{Map, Value};

#[derive(Debug)]
enum PathVertex {
    Internal(HashMap<String, PathVertex>),
    External,
}

fn path_map(paths: &HashSet<&str>) -> HashMap<String, PathVertex> {
    let mut output = HashMap::new();
    let mut collected_children = HashMap::new();

    let mut paths = Vec::from_iter(paths.clone());
    // sort this so that if we have a "*", it comes first
    paths.sort_unstable();

    for path in paths {
        let (parent, child) = match path.split_once("/") {
            Some((parent, child)) => (parent, Some(child)),
            None => (path, None),
        };

        // if we have a "*", store everything under it
        let children = match collected_children.get_mut("*") {
            Some(children) => children,
            None => collected_children
                .entry(parent.to_owned())
                .or_insert_with(Vec::new),
        };

        if let Some(child) = child {
            children.push(child);
        }
    }

    for (parent, children) in collected_children.into_iter() {
        output.insert(
            parent,
            match children.is_empty() {
                true => PathVertex::External,
                false => PathVertex::Internal(path_map(&HashSet::from_iter(children))),
            },
        );
    }

    output
}

fn prune(paths: &HashMap<String, PathVertex>, value: Value) -> Option<Value> {
    let all = paths.get("*");
    let empty = HashMap::new();

    if let Value::Object(map_old) = value {
        let mut map_new = Map::new();

        for (key, value) in map_old.into_iter() {
            if let Some(paths) = all.or_else(|| paths.get(&key)) {
                let paths = match paths {
                    // this branch of permissions expects more tree
                    PathVertex::Internal(paths) => paths,
                    // this branch of permissions expects the end of a tree
                    PathVertex::External => &empty,
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
        let paths = path_map(&HashSet::from(["users/*/nick", "users/00AAAAAAG/nick"]));

        if let Some(value) = prune(&paths, serde_json::to_value(network)?) {
            Ok(value.to_string())
        } else {
            Ok("{}".to_string())
        }
    }
}
