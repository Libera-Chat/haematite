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

mod ban;
mod channel;
mod handler;
mod hostmask;
mod line;
mod mode;
mod network;
mod oper;
mod server;
mod user;
mod util;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use colored::{Color, Colorize};

use crate::handler::ts6::TS6Handler;
use crate::handler::{Handler, Outcome};
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;
use crate::util::DecodeHybrid;

const PASSWORD: &str = "8m1RXdPW2HG8lakqJF53N6DYZRA6xRy0ORjIqod65RWok482rhgBQUfNTYcaJorJ";

fn send(mut socket: &TcpStream, data: &str) {
    println!("> {}", data);
    socket.write_all(data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

struct Haematite<T: Handler> {
    network: Network,
    handler: T,
}

impl<T: Handler> Haematite<T> {
    fn new(me: Server, handler: T) -> Self {
        Haematite {
            network: Network::new(me),
            handler,
        }
    }

    pub fn handle(&mut self, line: Line) -> Result<Outcome, &'static str> {
        self.handler.handle(&mut self.network, line)
    }
}

fn main() {
    let mut haematite = Haematite::new(
        Server {
            sid: "111".to_string(),
            name: String::from("haematite.vpn.lolnerd.net"),
            description: String::from("haematite psuedoserver"),
            ..Server::default()
        },
        TS6Handler::new(),
    );

    let socket = TcpStream::connect("husky.vpn.lolnerd.net:6667").expect("failed to connect");

    match haematite.handler.get_burst(&haematite.network, PASSWORD) {
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

        let line = match Line::from(&buffer) {
            Ok(line) => line,
            Err(e) => {
                eprintln!("failed to parse line");
                eprintln!("  {}", DecodeHybrid::decode(&buffer));
                eprintln!("  {:?}", e);
                std::process::exit(2);
            }
        };

        let outcome = match haematite.handle(line) {
            Ok(outcome) => outcome,
            Err(e) => {
                eprintln!("failed to handle line");
                eprintln!("  {}", DecodeHybrid::decode(&buffer));
                eprintln!("  {}", e);
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
