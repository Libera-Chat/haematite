use std::io::Error as IoError;
use std::sync::Arc;

use colored::{Color, Colorize};
use haematite_models::config::Config;
use haematite_models::network::Network;
use haematite_s2s::handler::{Error as HandlerError, Handler, Outcome};
use haematite_s2s::DecodeHybrid;
use rustls::client::InvalidDnsNameError;
use tokio::io::{split, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use crate::tls::{make_config, Error as TlsError};

#[derive(Debug)]
pub enum Error {
    Host,
    Socket(IoError),
    Tls(TlsError),
    MakeBurst(String),
    HandleLine(String, HandlerError),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Socket(error)
    }
}

impl From<InvalidDnsNameError> for Error {
    fn from(_error: InvalidDnsNameError) -> Self {
        Self::Host
    }
}

impl From<TlsError> for Error {
    fn from(error: TlsError) -> Self {
        Self::Tls(error)
    }
}

async fn send<T>(socket: &mut T, data: &str) -> Result<(), Error>
where
    T: AsyncWrite + Unpin,
{
    println!("> {}", data);
    socket.write_all(data.as_bytes()).await?;
    socket.write_all(b"\r\n").await?;
    Ok(())
}

pub async fn run(
    config: &Config,
    network: &mut Network,
    mut handler: impl Handler,
) -> Result<(), Error> {
    let tconfig = make_config(&config.uplink.ca, &config.tls)?;
    let connector = TlsConnector::from(Arc::new(tconfig));

    let socket = TcpStream::connect((config.uplink.host.clone(), config.uplink.port)).await?;
    let socket = connector
        .connect(config.uplink.host.as_str().try_into()?, socket)
        .await?;
    let (socket_r, mut socket_w) = split(socket);

    let burst = handler
        .get_burst(network, &config.uplink.password)
        .map_err(Error::MakeBurst)?;
    for line in burst {
        send(&mut socket_w, &line).await?;
    }

    let mut reader = BufReader::with_capacity(512, socket_r);
    let mut buffer = Vec::<u8>::with_capacity(512);
    while let Ok(len) = reader.read_until(b'\n', &mut buffer).await {
        // chop off \r\n
        buffer.drain(len - 2..len);

        let outcome = handler
            .handle(network, &buffer)
            .map_err(|e| Error::HandleLine(DecodeHybrid::decode(&buffer), e))?;
        let printable = DecodeHybrid::decode(&buffer);
        let printable = match outcome {
            Outcome::Unhandled => printable.color(Color::Red),
            _ => printable.normal(),
        };
        println!("< {}", printable);

        if let Outcome::Response(resps) = outcome {
            for resp in resps {
                send(&mut socket_w, &resp).await?;
            }
        }

        buffer.clear();
    }

    Ok(())
}
