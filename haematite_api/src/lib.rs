use std::collections::HashSet;

use haematite_models::network::Network;
use serde_json::{Map, Value};

pub struct Api {}

#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Api {
    fn prune<'a>(path: &'a str, paths: &HashSet<&'a str>, value: Value) -> Option<(bool, Value)> {
        match value {
            Value::Object(map_old) => {
                let mut map_new = Map::new();

                for (key, value) in map_old.into_iter() {
                    let path = format!("{}/{}", path, key);
                    if let Some((external_vertex, value)) = Api::prune(&path, paths, value) {
                        if !external_vertex || paths.contains(path.as_str()) {
                            map_new.insert(key, value);
                        }
                    }
                }

                if map_new.is_empty() {
                    None
                } else {
                    let output = (false, Value::Object(map_new));
                    Some(output)
                }
            }
            _ => {
                let output = (true, value);
                Some(output)
            }
        }
    }

    pub fn get_network(network: &Network) -> Result<String, serde_json::Error> {
        let paths = HashSet::from(["/users/00AAAAAAG/nick"]);

        if let Some((_, value)) = Api::prune("", &paths, serde_json::to_value(network)?) {
            Ok(value.to_string())
        } else {
            Ok("{}".to_string())
        }
    }
}
