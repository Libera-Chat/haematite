use std::{fs::File, io::BufReader, path::Path, sync::Arc};

use rustls::{
    pki_types::{CertificateDer, InvalidDnsNameError, PrivateKeyDer},
    ClientConfig, RootCertStore,
};
use tokio::net::TcpStream;
use tokio_rustls::{client::TlsStream, TlsConnector};

#[derive(Debug)]
pub enum Error {
    Hostname(InvalidDnsNameError),
    ReadCertificate(std::io::Error),
    ReadPrivateKey(std::io::Error),
    ParseCertificate(std::io::Error),
    ParsePrivateKey(std::io::Error),
    NoCertificate,
    NoPrivateKey,
    VerifyServer(std::io::Error),
}

impl From<Error> for super::socket::Error {
    fn from(value: Error) -> Self {
        Self::Tls(value)
    }
}

trait FromPem<T> {
    fn from_pem(path: &Path) -> Result<T, Error>;
}

impl<'a> FromPem<Self> for CertificateDer<'a> {
    fn from_pem(path: &Path) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::ReadCertificate)?;
        let mut read = BufReader::new(file);
        let mut der = rustls_pemfile::certs(&mut read);
        der.next()
            .ok_or(Error::NoCertificate)?
            .map_err(Error::ParseCertificate)
    }
}

impl<'a> FromPem<Self> for PrivateKeyDer<'a> {
    fn from_pem(path: &Path) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::ReadPrivateKey)?;
        let mut read = BufReader::new(file);
        let der = rustls_pemfile::private_key(&mut read);
        der.map_err(Error::ParsePrivateKey)?
            .ok_or(Error::NoPrivateKey)
    }
}

fn make_config(
    server_ca: &Path,
    client_config: &haematite_models::config::Mtls,
) -> Result<ClientConfig, Error> {
    let mut ca_store = RootCertStore::empty();
    ca_store.add(CertificateDer::from_pem(server_ca)?).unwrap();

    let client_crt = CertificateDer::from_pem(&client_config.crt)?;
    let client_key = PrivateKeyDer::from_pem(&client_config.key)?;

    let config = ClientConfig::builder()
        .with_root_certificates(ca_store)
        .with_client_auth_cert(vec![client_crt], client_key)
        .unwrap();

    Ok(config)
}

pub async fn wrap_socket(
    host: String,
    tcp_socket: TcpStream,
    mtls: &haematite_models::config::Mtls,
) -> Result<TlsStream<TcpStream>, Error> {
    let tconfig = make_config(&mtls.ca, mtls)?;
    let connector = TlsConnector::from(Arc::new(tconfig));
    let tls_socket: tokio_rustls::client::TlsStream<TcpStream> = connector
        .connect(host.try_into().map_err(Error::Hostname)?, tcp_socket)
        .await
        .map_err(Error::VerifyServer)?;

    Ok(tls_socket)
}
