use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use haematite_api::Api;
use haematite_models::config::Config;
use haematite_models::network::Network;
use warp::Filter;

pub async fn run(config: &Config, network_lock: Arc<RwLock<Network>>) -> Result<(), Infallible> {
    let path = warp::any().map(move || {
        let network = network_lock.read().unwrap();
        Api::get_network(&network).unwrap()
    });

    warp::serve(path).run(config.bind).await;

    Ok(())
}
