use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;

use rustls::{Certificate, ClientConfig, ClientConnection, RootCertStore};

pub fn make_connection(hostname: &str, ca: PathBuf) -> ClientConnection {
    let mut ca_store = RootCertStore::empty();
    let file = File::open(ca).expect("couldn't read CA");
    let ca_certs = rustls_pemfile::certs(&mut BufReader::new(file)).expect("couldn't decode PEM");
    for ca_cert in ca_certs {
        ca_store
            .add(&Certificate(ca_cert.clone()))
            .expect("couldn't load CA");
    }

    let config = Arc::new(
        ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(ca_store)
            .with_no_client_auth(),
    );

    ClientConnection::new(config, hostname.try_into().unwrap()).unwrap()
}
