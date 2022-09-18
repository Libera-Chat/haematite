use std::convert::Infallible;
use std::sync::{Arc, Mutex, RwLock};

use closure::closure;
use futures::{SinkExt as _, StreamExt as _};
use haematite_api::{Api, Format};
use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use serde_json::Value;
use tokio::sync::broadcast::Receiver;
use warp::ws::{Message, Ws};
use warp::Filter;

pub async fn run(
    config: &Config,
    network: Arc<RwLock<Network>>,
    stream: Receiver<(String, Value)>,
) -> Result<(), Infallible> {
    let stream = Arc::new(Mutex::new(stream));
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

    let path_stream = warp::path("stream").and(warp::ws()).map(move |ws1: Ws| {
        let mut stream = stream.lock().unwrap().resubscribe();
        ws1.on_upgrade(|ws2| async move {
            let (mut tx, rx) = ws2.split();

            while let Ok((path, value)) = stream.recv().await {
                if let Err(_) = tx.send(Message::text(format!("{} {}", path, value))).await {
                    break;
                }
            }
        })
    });

    warp::serve(path_network.or(path_user).or(path_stream))
        .run(config.bind)
        .await;

    Ok(())
}
