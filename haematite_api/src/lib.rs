use haematite_models::network::Network;

pub struct Api {}

#[derive(Debug)]
pub enum Error {
    Bad,
}

impl Api {
    pub fn get_network(network: &Network) -> Result<String, serde_json::Error> {
        serde_json::to_string(network)
    }
}
