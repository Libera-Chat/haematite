use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

#[derive(Debug)]
pub enum Error {
    Connect,
    Tls(super::tls::Error),
    Send(std::io::Error),
}

pub async fn make(
    uplink: &haematite_models::config::Uplink,
    mtls: &haematite_models::config::Mtls,
) -> Result<TlsStream<TcpStream>, Error> {
    let tcp_socket = TcpStream::connect((uplink.host.clone(), uplink.port))
        .await
        .map_err(|_| Error::Connect)?;

    let tls_socket = super::tls::wrap_socket(uplink.host.clone(), tcp_socket, mtls).await?;

    Ok(tls_socket)
}

pub async fn send<T: AsyncWrite + Send + Unpin>(
    socket: &mut T,
    data: &str,
    verbose: u8,
) -> Result<(), Error> {
    if verbose > 1 {
        println!("> {data}");
    }
    socket
        .write_all(data.as_bytes())
        .await
        .map_err(Error::Send)?;
    socket.write_all(b"\r\n").await.map_err(Error::Send)?;
    Ok(())
}
