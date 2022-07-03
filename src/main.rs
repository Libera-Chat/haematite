mod channel;
mod channel_user;
mod line;
mod network;
mod server;
mod user;

use channel::Channel;
use line::Line;
use network::Network;
use server::Server;
use user::User;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

struct Haematite {
    network: Network,
    me: Server,
    uplink: Option<String>,
}

impl Haematite {
    fn new(me: Server) -> Self {
        Haematite {
            network: Network::new(),
            me,
            uplink: None,
        }
    }

    fn handle_line(&mut self, socket: &TcpStream, line: &Line) -> bool {
        match line.command {
            "PASS" => self.uplink = Some(line.args[3].to_string()),
            "SERVER" => {
                self.network.add_server(Server {
                    sid: self.uplink.take().unwrap(),
                    name: line.args[0].to_string(),
                    description: line.args[2].to_string(),
                    ..Default::default()
                });
            }
            "SID" => {
                let server = Server {
                    sid: line.args[2].to_string(),
                    name: line.args[0].to_string(),
                    description: line.args[3].to_string(),
                    ..Default::default()
                };
                self.network.add_server(server);
            }
            "SQUIT" => {
                let sid = line.args[0];
                self.network.del_server(sid);
            }
            //:00A EUID alis 2 1656866967 +Sio alis atheme.vpn.lolnerd.net 0 00AAAAAAB * * :Channel Directory
            "EUID" => {
                let server = self.network.get_server_mut(line.source.unwrap());
                server.add_user(User {
                    uid: line.args[7].to_string(),
                    nickname: line.args[0].to_string(),
                });
            }
            "SJOIN" => {
                //:420 SJOIN 1640815917 #gaynet +MOPnst :@00AAAAAAC 420AAAABC
                let name = line.args[1].to_string();
                let users = line.args[3].split(' ');
                self.network.add_channel(name, Channel::new(users));
            }
            "PING" => {
                let source = match line.source {
                    Some(source) => source,
                    None => line.args[0],
                };
                send(
                    &socket,
                    format!(":{} PONG {} {}", self.me.sid, self.me.name, source),
                );
            }
            "NOTICE" => { /* silently eat */ }
            _ => {
                return false;
            }
        }
        return true;
    }
}

const PASSWORD: &str = "8m1RXdPW2HG8lakqJF53N6DYZRA6xRy0ORjIqod65RWok482rhgBQUfNTYcaJorJ";

fn send(mut socket: &TcpStream, data: String) {
    println!("> {}", data);
    socket.write_all(&data.as_bytes()).expect("asd");
    socket.write_all(b"\r\n").expect("asd");
}

fn main() {
    let mut haematite = Haematite::new(Server {
        sid: String::from("111"),
        name: String::from("haematite.vpn.lolnerd.net"),
        description: String::from("haematite psuedoserver"),
        ..Default::default()
    });

    let socket = TcpStream::connect("husky.vpn.lolnerd.net:6667").expect("failed to connect");

    send(
        &socket,
        format!("PASS {} TS 6 :{}", PASSWORD, haematite.me.sid),
    );
    send(
        &socket,
        "CAPAB :BAN CHW CLUSTER ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN".to_string(),
    );
    send(
        &socket,
        format!(
            "SERVER {} 1 :{}",
            haematite.me.name, haematite.me.description
        ),
    );

    let mut reader = BufReader::with_capacity(512, &socket);
    let mut buffer = Vec::<u8>::with_capacity(512);
    loop {
        let len = reader.read_until(b'\n', &mut buffer).unwrap_or(0);
        if len == 0 {
            break;
        }

        // chop off \r\n
        buffer.drain(len - 2..len);

        let line = Line::from(&buffer);
        if !haematite.handle_line(&socket, &line) {
            // only print lines we don't understand
            println!("< {}", from_utf8(&buffer).unwrap().to_owned());
        }

        buffer.clear();
    }
}
