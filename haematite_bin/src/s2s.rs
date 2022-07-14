use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use colored::{Color, Colorize};
use haematite_models::config::Config;
use haematite_models::network::Network;
use haematite_s2s::handler::{Error as HandlerError, Handler, Outcome};
use haematite_s2s::DecodeHybrid;
use rustls::Stream;

use crate::tls::make_connection;

#[derive(Debug)]
pub enum Error {
    MakeBurst(String),
    HandleLine(String, HandlerError),
}

fn send(socket: &mut impl Write, data: &str) {
    println!("> {}", data);
    socket.write_all(data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

/// # Errors
///
/// Errors if data read from socket cannot be decoded.
pub fn run(config: &Config, network: &mut Network, mut handler: impl Handler) -> Result<(), Error> {
    let mut psocket = TcpStream::connect((config.uplink.host.clone(), config.uplink.port))
        .expect("failed to connect");
    let mut connection =
        make_connection(&config.uplink.host, &config.uplink.ca, &config.tls).unwrap();
    let mut socket = Stream::new(&mut connection, &mut psocket);

    let burst = handler
        .get_burst(network, &config.uplink.password)
        .map_err(Error::MakeBurst)?;
    for line in burst {
        send(&mut socket, &line);
    }

    let mut reader = BufReader::with_capacity(512, socket);
    let mut buffer = Vec::<u8>::with_capacity(512);
    while let Ok(len) = reader.read_until(b'\n', &mut buffer) {
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
                send(reader.get_mut(), &resp);
            }
        }

        buffer.clear();
    }

    Ok(())
}
