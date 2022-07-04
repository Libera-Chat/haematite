mod channel;
mod channel_user;
mod handler;
mod line;
mod network;
mod server;
mod user;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use colored::{Color, Colorize};

use crate::handler::ts6::TS6Handler;
use crate::handler::Handler;
use crate::line::Line;
use crate::network::Network;
use crate::server::Server;

const PASSWORD: &str = "8m1RXdPW2HG8lakqJF53N6DYZRA6xRy0ORjIqod65RWok482rhgBQUfNTYcaJorJ";

fn send(mut socket: &TcpStream, data: String) {
    println!("> {}", data);
    socket.write_all(data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

struct Haematite<T>
where
    T: Handler,
{
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

    pub fn handle(&mut self, socket: &TcpStream, line: &Line) -> bool {
        self.handler.handle(&mut self.network, socket, line)
    }
}

fn main() {
    let mut haematite = Haematite::new(
        Server {
            sid: String::from("111"),
            name: String::from("haematite.vpn.lolnerd.net"),
            description: String::from("haematite psuedoserver"),
            ..Default::default()
        },
        TS6Handler::new(),
    );

    let socket = TcpStream::connect("husky.vpn.lolnerd.net:6667").expect("failed to connect");

    for line in haematite.handler.get_burst(&haematite.network, PASSWORD) {
        send(&socket, line);
    }

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
                eprintln!("failed to parse line: {:?}", e);
                std::process::exit(1);
            }
        };
        let handled = haematite.handle(&socket, &line);

        let printable = from_utf8(&buffer).unwrap().to_string();
        println!(
            "< {}",
            match handled {
                true => printable.normal(),
                false => printable.color(Color::Red),
            }
        );

        buffer.clear();
    }
}
