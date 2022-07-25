use std::convert::Infallible;
use std::sync::{Arc, RwLock};

use haematite_api::Api;
use haematite_models::network::Network;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Response, Server};

pub async fn run(network_lock: Arc<RwLock<Network>>) {
    let make_svc = make_service_fn(move |_conn| {
        let network_lock = Arc::clone(&network_lock);
        async move {
            Ok::<_, Infallible>(service_fn(move |_req| {
                let network_lock = Arc::clone(&network_lock);
                async move {
                    let response = {
                        let network = network_lock.read().unwrap();
                        match Api::get_network(&network) {
                            Ok(it) => it,
                            Err(e) => {
                                println!("{:?}", e);
                                std::process::exit(123);
                            }
                        }
                    };
                    Ok::<_, Infallible>(
                        Response::builder()
                            .header("content-type", "application/json")
                            .body(response)
                            .unwrap(),
                    )
                }
            }))
        }
    });

    let bind = "[fd84:9d71:8b8:1::1]:8085".parse().unwrap();
    let server = Server::bind(&bind).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
