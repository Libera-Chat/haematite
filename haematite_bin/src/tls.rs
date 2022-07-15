use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use haematite_models::config::Tls;
use rustls::{Certificate, ClientConfig, ClientConnection, PrivateKey, RootCertStore};

#[derive(Debug)]
pub enum Error {
    Read,
    Parse,
}

trait FromPem<T> {
    fn from_pem(path: &Path) -> Result<T, Error>;
}

impl FromPem<Certificate> for Certificate {
    fn from_pem(path: &Path) -> Result<Certificate, Error> {
        let file = File::open(path).map_err(|_| Error::Read)?;
        let mut read = BufReader::new(file);
        let der = &rustls_pemfile::certs(&mut read).map_err(|_| Error::Parse)?[0];
        Ok(Certificate(der.clone()))
    }
}

impl FromPem<PrivateKey> for PrivateKey {
    fn from_pem(path: &Path) -> Result<PrivateKey, Error> {
        let file = File::open(path).map_err(|_| Error::Read)?;
        let mut read = BufReader::new(file);
        let der = &rustls_pemfile::pkcs8_private_keys(&mut read).map_err(|_| Error::Parse)?[0];
        Ok(PrivateKey(der.clone()))
    }
}

pub fn make_connection(
    server_host: &str,
    server_ca: &Path,
    client_config: &Tls,
) -> Result<ClientConnection, Error> {
    let mut ca_store = RootCertStore::empty();
    ca_store.add(&Certificate::from_pem(server_ca)?).unwrap();

    let client_crt = Certificate::from_pem(&client_config.crt)?;
    let client_key = PrivateKey::from_pem(&client_config.key)?;

    let config = Arc::new(
        ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(ca_store)
            .with_single_cert(vec![client_crt], client_key)
            .unwrap(),
    );

    Ok(ClientConnection::new(config, server_host.try_into().unwrap()).unwrap())
}
