use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use closure::closure;
use futures::stream::SplitSink;
use futures::{SinkExt as _, StreamExt as _};
use haematite_api::{Api, Format};
use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use serde_json::Value;
use tokio::sync::broadcast::Receiver;
use tokio::sync::RwLock as AsyncRwLock;
use warp::ws::{Message, WebSocket, Ws};
use warp::Filter;

struct WebsocketHandler {
    stream: Receiver<(String, Value)>,
    sockets: Arc<AsyncRwLock<HashMap<String, SplitSink<WebSocket, Message>>>>,
}

impl WebsocketHandler {
    pub async fn run(&mut self) {
        while let Ok((path, value)) = self.stream.recv().await {
            for (_, socket) in self.sockets.write().await.iter_mut() {
                socket
                    .send(Message::text(format!("{} {}", path, value)))
                    .await
                    .unwrap();
            }
        }
    }
}

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

    let sockets = Arc::new(AsyncRwLock::new(HashMap::new()));
    let mut websocket_handler = WebsocketHandler {
        stream,
        sockets: Arc::clone(&sockets),
    };

    let path_stream = warp::path("stream").and(warp::ws()).map(
        closure!(clone sockets, |ws1: Ws| ws1.on_upgrade(closure!(clone sockets, |ws2| async move {
            let (tx, mut rx) = ws2.split();
            sockets.write().await.insert("asd".to_string(), tx);
            while let Some(_message) = rx.next().await {

            }
        }))),
    );

    tokio::join!(
        warp::serve(path_network.or(path_user).or(path_stream)).run(config.bind),
        websocket_handler.run(),
    );

    Ok(())
}
