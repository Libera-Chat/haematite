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

mod config;
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
use haematite_models::network::Network;

use crate::config::Config;
use crate::handler::{Handler, Outcome};
use crate::ts6::TS6Handler;
use crate::util::DecodeHybrid;

use clap::Parser;

fn send(mut socket: &TcpStream, data: &str) {
    println!("> {}", data);
    socket.write_all(data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to config file
    #[clap(index = 1)]
    config: std::path::PathBuf,
}

fn main_inner(config: Config, network: &mut Network) {
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
    loop {
        let len = reader.read_until(b'\n', &mut buffer).unwrap_or(0);
        if len == 0 {
            break;
        }

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

        let printable = from_utf8(&buffer).unwrap().to_string();
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
}

fn main() {
    let args = CliArgs::parse();

    let config = match Config::from_file(args.config) {
        Ok(it) => it,
        Err(err) => {
            eprintln!("failed to read config file: {}", err);
            std::process::exit(1);
        }
    };

    let mut network = Network::new(config.server.clone());
    main_inner(config, &mut network);
}
