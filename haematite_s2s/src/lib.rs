#![deny(clippy::pedantic)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::equatable_if_let)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::same_name_method)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::shadow_unrelated)]
#![allow(clippy::similar_names)]

mod handler;
mod line;
mod mode;
mod rfc1459;
mod ts6;
mod util;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use colored::{Color, Colorize};
use haematite_models::config::Config;
use haematite_models::network::Network;

use crate::handler::{Error as HandlerError, Handler, Outcome};
use crate::ts6::TS6Handler;
use crate::util::DecodeHybrid;

fn send(mut socket: &TcpStream, data: &str) {
    println!("> {}", data);
    socket.write_all(data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

/// # Errors
///
/// Errors if data read from socket cannot be decoded.
pub fn main(config: Config, network: &mut Network) -> Result<(), HandlerError> {
    let mut handler = TS6Handler::new();
    handler.validate_config(&config).expect("invalid config");

    let socket =
        TcpStream::connect((config.uplink.host, config.uplink.port)).expect("failed to connect");

    match handler.get_burst(network, &config.uplink.password) {
        Err(burst_err) => {
            eprintln!("failed to make burst: {}", burst_err);
            std::process::exit(1);
        }
        Ok(burst) => {
            for line in burst {
                send(&socket, &line);
            }
        }
    };

    let mut reader = BufReader::with_capacity(512, &socket);
    let mut buffer = Vec::<u8>::with_capacity(512);
    while let Ok(len) = reader.read_until(b'\n', &mut buffer) {
        // chop off \r\n
        buffer.drain(len - 2..len);

        let outcome = match handler.handle(network, &buffer) {
            Ok(outcome) => outcome,
            Err(e) => {
                eprintln!("failed to handle line");
                eprintln!("  {}", DecodeHybrid::decode(&buffer));
                eprintln!("  {:?}", e);
                std::process::exit(3);
            }
        };

        let printable = from_utf8(&buffer)
            .map_err(|_| HandlerError::InvalidProtocol)?
            .to_string();
        let printable = match outcome {
            Outcome::Unhandled => printable.color(Color::Red),
            _ => printable.normal(),
        };
        println!("< {}", printable);

        if let Outcome::Response(resps) = outcome {
            for resp in resps {
                send(&socket, &resp);
            }
        }

        buffer.clear();
    }

    Ok(())
}
