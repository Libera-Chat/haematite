use std::io::Error as IoError;
use std::sync::{Arc, RwLock};

use colored::{Color, Colorize};
use rustls::client::InvalidDnsNameError;
use tokio::io::{split, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_rustls::TlsConnector;

use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use haematite_models::meta::permissions::Path;
use haematite_s2s::handler::{Error as HandlerError, Handler, Outcome};
use haematite_s2s::DecodeHybrid;
use haematite_ser::{Serializer, WrapType};

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
    network_lock: Arc<RwLock<Network>>,
    state_stream: mpsc::Sender<(Path, WrapType)>,
    mut handler: impl Handler,
) -> Result<(), Error> {
    let tconfig = make_config(&config.uplink.ca, &config.tls)?;
    let connector = TlsConnector::from(Arc::new(tconfig));

    let socket = TcpStream::connect((config.uplink.host.clone(), config.uplink.port)).await?;
    let socket = connector
        .connect(config.uplink.host.as_str().try_into()?, socket)
        .await?;

    let (socket_r, mut socket_w) = split(socket);

    let burst = {
        let network = network_lock.read().unwrap();
        handler
            .get_burst(&network, &config.uplink.password)
            .map_err(Error::MakeBurst)?
    };

    for line in burst {
        send(&mut socket_w, &line).await?;
    }

    let mut reader = BufReader::with_capacity(512, socket_r);
    let mut buffer = Vec::<u8>::with_capacity(512);
    while let Ok(len) = reader.read_until(b'\n', &mut buffer).await {
        // chop off \r\n
        buffer.drain(len - 2..len);

        let outcome = {
            let network = network_lock.read().unwrap();
            handler
                .handle(&network, &buffer)
                .map_err(|e| Error::HandleLine(DecodeHybrid::decode(&buffer), e))?
        };

        let printable = DecodeHybrid::decode(&buffer);
        match outcome {
            Outcome::Unhandled => println!("< {}", printable.color(Color::Red)),
            Outcome::State(diffs) => {
                println!("< {}", printable);
                let mut diffs_out = Vec::new();

                {
                    let mut network = network_lock.write().unwrap();

                    for diff in diffs {
                        diffs_out.push(network.update(diff, &mut Serializer {}).unwrap());
                    }

                    // scope to make `network` is dropped before below await
                }

                for (path, value) in diffs_out {
                    println!(
                        "{} {}",
                        path.to_string().color(Color::Blue),
                        serde_json::to_string(&value).unwrap()
                    );
                    state_stream.send((path, value)).await.unwrap();
                }
            }
            Outcome::Response(responses) => {
                println!("< {}", printable);
                for response in responses {
                    send(&mut socket_w, &response).await?;
                }
            }
            Outcome::Empty => println!("< {}", printable),
        };

        buffer.clear();
    }

    Ok(())
}
