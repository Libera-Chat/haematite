use std::convert::Infallible;
use std::future::Future;
use std::sync::{Arc, RwLock};

use haematite_api::Api;
use haematite_models::network::Network;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

struct Handler {
    network_lock: Arc<RwLock<Network>>,
}

impl Handler {
    fn on_request(
        &self,
        _request: &Request<Body>,
    ) -> impl Future<Output = Result<Response<String>, Infallible>> {
        let network = self.network_lock.read().unwrap();
        let response = Response::builder()
            .header("content-type", "application/json")
            .body(Api::get_network(&network).unwrap())
            .unwrap();
        async { Ok(response) }
    }
}

pub async fn run(network_lock: Arc<RwLock<Network>>) {
    let bind = "[fd84:9d71:8b8:1::1]:8085".parse().unwrap();
    let server = Server::bind(&bind).serve(make_service_fn(|_stream| {
        let handler = Handler {
            network_lock: Arc::clone(&network_lock),
        };
        async move { Ok::<_, Infallible>(service_fn(move |req| handler.on_request(&req))) }
    }));

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
