use std::time::Instant;

use colored::{Color, Colorize};
use tokio::io::{split, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;

use haematite_models::config::Config;
use haematite_models::irc::network::Network;
use haematite_s2s::handler::{Error as HandlerError, Handler, Outcome};
use haematite_s2s::DecodeHybrid;

type EventSenderError = tokio::sync::mpsc::error::SendError<(&'static str, Vec<u8>)>;

#[derive(Debug)]
pub enum Error {
    Socket(crate::util::socket::Error),
    MakeBurst(String),
    HandleLine(Vec<u8>, HandlerError),
    EventSender(EventSenderError),
}

impl From<EventSenderError> for Error {
    fn from(value: EventSenderError) -> Self {
        Self::EventSender(value)
    }
}

impl From<crate::util::socket::Error> for Error {
    fn from(value: crate::util::socket::Error) -> Self {
        Self::Socket(value)
    }
}

#[allow(clippy::too_many_lines)]
pub async fn run<H: Handler + Send>(
    config: &Config,
    mut network: Network,
    mut handler: H,
    event_queue: Sender<(&'static str, Vec<u8>)>,
    verbose: u8,
) -> Result<(), Error> {
    let socket = crate::util::socket::make(&config.uplink, &config.mtls).await?;
    let (socket_r, mut socket_w) = split(socket);

    let burst = handler
        .get_burst(&config.uplink.password)
        .map_err(Error::MakeBurst)?;

    for line in burst {
        crate::util::socket::send(&mut socket_w, &line, verbose).await?;
    }

    let mut event_store = haematite_events::event_store::json::EventStore::default();
    let mut reader = BufReader::new(socket_r);
    let mut buffer = Vec::with_capacity(512);
    while let Ok(len) = reader.read_until(b'\n', &mut buffer).await {
        if len == 0 {
            break;
        } else if len > 512 {
            println!("too big!");
            break;
        }
        let line = &buffer[..len - 2];

        if verbose == 1 {
            if let Ok(line) = std::str::from_utf8(line) {
                println!("{line}");
            }
        }

        let now = Instant::now();
        let outcome = handler
            .handle(&mut event_store, &mut network, line)
            .map_err(|e| Error::HandleLine(line.to_vec(), e))?;

        if verbose > 0 {
            let elapsed = now.elapsed().as_nanos();
            println!(
                "line handled in {}.{:0>3}µs",
                elapsed / 1000,
                elapsed % 1000
            );
        }

        match outcome {
            Outcome::Unhandled => {
                if verbose > 1 {
                    let printable = DecodeHybrid::decode(line);
                    println!("< {}", printable.color(Color::Red));
                }
            }
            Outcome::Handled => {
                if verbose > 1 {
                    let printable = DecodeHybrid::decode(line);
                    println!("< {printable}");
                }

                while let Some((item_type, payload)) = event_store.payloads.pop_front() {
                    if verbose > 2 {
                        println!("{item_type} {:}", std::str::from_utf8(&payload).unwrap());
                    }
                    let now = Instant::now();
                    event_queue.send((item_type, payload)).await?;
                    if verbose > 0 {
                        let elapsed = now.elapsed().as_nanos();
                        println!(
                            "event published in {}.{:0>3}us",
                            elapsed / 1000,
                            elapsed % 1000
                        );
                    }
                }
            }
            Outcome::Responses(responses) => {
                if verbose > 1 {
                    let printable = DecodeHybrid::decode(line);
                    println!("< {printable}");
                }
                for response in responses {
                    crate::util::socket::send(&mut socket_w, &response, verbose).await?;
                }
            }
        };

        buffer.clear();
    }

    Ok(())
}
