use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use closure::closure;
use haematite_api::{Api, Format};
use haematite_models::config::Config;
use haematite_models::network::Network;
use warp::Filter;

pub async fn run(config: &Config, network: Arc<RwLock<Network>>) -> Result<(), Infallible> {
    let api = Arc::new(Api::new(Format::Pretty));
    let root = warp::path("rest");

    let path_network = warp::path("network").map(closure!(clone network, clone api, || {
        let network = network.read().unwrap();
        api.get_network(&network).unwrap()
    }));

    let path_user =
        warp::path!("user" / String).map(closure!(clone network, clone api, |uid: String| {
            let network = network.read().unwrap();
            api.get_user(&network, &uid).unwrap()
        }));

    let paths = path_network.or(path_user);

    warp::serve(root.and(paths)).run(config.bind).await;

    Ok(())
}
