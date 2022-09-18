use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use closure::closure;
use futures::StreamExt as _;
use haematite_api::{Api, Format};
use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use serde_json::Value;
use tokio::sync::broadcast::Receiver;
use warp::ws::Ws;
use warp::Filter;

pub async fn run(
    config: &Config,
    network: Arc<RwLock<Network>>,
    stream: Receiver<(String, Value)>,
) -> Result<(), Infallible> {
    let api = Arc::new(Api::new(Format::Pretty));

    let path_network = warp::path!("rest" / "network").map(closure!(clone network, clone api, || {
        let network = network.read().unwrap();
        api.get_network(&network).unwrap()
    }));

    let path_user = warp::path!("rest" / "user" / String).map(
        closure!(clone network, clone api, |uid: String| {
            let network = network.read().unwrap();
            api.get_user(&network, &uid).unwrap()
        }),
    );

    let path_stream = warp::path("stream").and(warp::ws()).map(|ws1: Ws| {
        // need to do this :((
        //let stream = stream.resubscribe();
        ws1.on_upgrade(|ws2| async {
            let (tx, mut rx) = ws2.split();
            while let Some(_message) = rx.next().await {}
        })
    });

    warp::serve(path_network.or(path_user).or(path_stream))
        .run(config.bind)
        .await;

    Ok(())
}
