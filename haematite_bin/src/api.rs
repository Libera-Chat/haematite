use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use haematite_api::Api;
use haematite_models::network::Network;
use warp::Filter;

#[derive(Debug)]
pub enum Error {
    Bind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub async fn run(network_lock: Arc<RwLock<Network>>) -> Result<(), Error> {
    let path = warp::any().map(move || {
        let network = network_lock.read().unwrap();
        Api::get_network(&network).unwrap()
    });

    let bind = "[fd84:9d71:8b8:1::1]:8085"
        .parse::<SocketAddr>()
        .map_err(|_| Error::Bind)?;

    warp::serve(path).run(bind).await;

    Ok(())
}
