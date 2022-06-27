mod network;
mod server;

use network::Network;
use server::Server;

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

const ME: Server = Server {
    sid: "111",
    name: "haematite.vpn.lolnerd.net",
    description: "haematite pseudoserver",
};
const PASSWORD: &str = "8m1RXdPW2HG8lakqJF53N6DYZRA6xRy0ORjIqod65RWok482rhgBQUfNTYcaJorJ";

fn send(mut socket: &TcpStream, data: String) {
    println!("> {}", data);
    socket.write_all(&data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

struct Line<'a> {
    source: Option<&'a str>,
    command: &'a str,
    args: Vec<&'a str>,
}
fn tokenise_line(line_full: &str) -> Line {
    let mut offset = 0;

    let source = match line_full.chars().next() {
        Some(':') => {
            offset = line_full.find(' ').unwrap() + 1;
            Some(&line_full[0..offset - 1])
        }
        _ => None,
    };

    let mut line = &line_full[offset..];
    let trailing = match line.find(" :") {
        Some(i) => {
            let out = Some(&line[i + 2..]);
            line = &line[..i];
            out
        }
        _ => None,
    };

    let mut args: VecDeque<&str> = line.split(' ').collect();
    match trailing {
        Some(s) => args.push_back(s),
        None => {}
    };

    Line {
        source: source,
        command: args.pop_front().unwrap(),
        args: args.into(),
    }
}

fn handle_line(network: &mut Network, socket: &TcpStream, line: Line) {
    match line.command {
        "SID" => {
            let server = Server {
                sid: line.args[2],
                name: line.args[0],
                description: line.args[3],
            };
            network.add_server(&server);
        }
        "PING" => {
            let source = match line.source {
                Some(source) => source,
                None => line.args[0],
            };
            send(&socket, format!(":{} PONG {} {}", ME.sid, ME.name, source));
        }
        _ => {}
    }
}

fn main() {
    let mut network = Network::new();
    network.add_server(&ME);

    let socket = TcpStream::connect("husky.vpn.lolnerd.net:6667").expect("failed to connect");

    send(&socket, format!("PASS {} TS 6 :{}", PASSWORD, ME.sid));
    send(
        &socket,
        "CAPAB :BAN CHW CLUSTER ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN".to_string(),
    );
    send(&socket, format!("SERVER {} 1 :{}", ME.name, ME.description));

    let mut reader = BufReader::with_capacity(512, &socket);
    let mut buffer = Vec::<u8>::with_capacity(512);
    loop {
        let len = reader.read_until(b'\n', &mut buffer).unwrap_or(0);
        if len == 0 {
            break;
        }

        // chop off \r\n
        buffer.drain(len - 1..len);
        let line_decode = from_utf8(&buffer).unwrap();
        println!("< {}", line_decode);
        let line_tokenised = tokenise_line(line_decode);
        handle_line(&mut network, &socket, line_tokenised);
        buffer.clear();
    }
}
