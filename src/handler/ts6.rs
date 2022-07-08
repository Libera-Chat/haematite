use std::time::SystemTime;

use crate::ban::Ban;
use crate::channel::Channel;
use crate::handler::{Handler, Outcome};
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::oper::Oper;
use crate::server::Server;
use crate::user::User;

fn mode_args(
    modes: impl Iterator<Item = (char, bool)>,
    args: impl IntoIterator<Item = String>,
) -> impl Iterator<Item = (char, bool, Option<String>)> {
    let mut out = Vec::new();
    let mut args = args.into_iter();

    for (mode, remove) in modes {
        let arg = match mode {
            'k' => true,
            'f' | 'j' | 'l' if !remove => true,
            _ => false,
        }
        //TODO: Resultify this unwrap
        .then(|| args.next().unwrap());

        out.push((mode, remove, arg));
    }

    out.into_iter()
}

#[derive(Default)]
pub struct TS6Handler {
    uplink: Option<String>,
}

impl TS6Handler {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_line_none(&mut self, network: &mut Network, line: &Line) -> Outcome {
        match line.command.as_slice() {
            b"PASS" => self.uplink = Some(line.args[3].to_string()),
            b"SERVER" => {
                network.add_server(Server {
                    sid: self.uplink.take().unwrap(),
                    name: line.args[0].to_string(),
                    description: line.args[2].to_string(),
                    ..Server::default()
                });
            }
            b"PING" => {
                return Outcome::Response(vec![format!(
                    ":{} PONG {} {}",
                    network.me.sid, network.me.name, line.args[0]
                )]);
            }
            _ => {
                return Outcome::Unhandled;
            }
        }

        Outcome::Empty
    }

    fn handle_line_encap(
        network: &mut Network,
        _sid: &str,
        _target: &str,
        command: &str,
        args: &[String],
    ) -> Outcome {
        match command {
            //:00A ENCAP * SU :420AAAABF
            //:00A ENCAP * SU 420AAAABF :jess
            "SU" => {
                let uid = &args[0];
                let server = network.get_server_mut(&uid[..3]);
                server.get_user_mut(uid).account = args.get(1).map(ToString::to_string);
            }
            _ => {
                return Outcome::Unhandled;
            }
        }
        Outcome::Empty
    }

    fn handle_line_sid(network: &mut Network, sid: &str, line: &Line) -> Outcome {
        match line.command.as_slice() {
            b"SID" => {
                network.add_server(Server {
                    sid: line.args[2].to_string(),
                    name: line.args[0].to_string(),
                    description: line.args[3].to_string(),
                    ..Server::default()
                });
            }
            b"SQUIT" => {
                let sid = &line.args[0];
                network.del_server(sid);
            }
            //:420 EUID jess 1 1656880345 +QZaioswz a0Ob4s0oLV test. fd84:9d71:8b8:1::1 420AAAABD husky.vpn.lolnerd.net jess :big meow
            b"EUID" => {
                let uid = line.args[7].to_string();
                let nickname = line.args[0].to_string();
                let username = line.args[4].to_string();
                let realname = line.args[10].to_string();
                let account = match line.args[9].as_str() {
                    "*" => None,
                    account => Some(account.to_string()),
                };
                let ip = match line.args[6].as_str() {
                    "0" => None,
                    ip => Some(ip.to_string()),
                };
                let rdns = match line.args[8].as_str() {
                    "*" => None,
                    rdns => Some(rdns.to_string()),
                };
                let host = line.args[5].to_string();

                let server = network.get_server_mut(sid);
                let mut user = User::new(nickname, username, realname, account, ip, rdns, host);

                for (mode, _) in modes_from(&line.args[3]) {
                    user.modes.insert(mode);
                }

                server.add_user(uid, user);
            }
            //:00A CHGHOST 420AAAABD husky.vpn.lolnerd.net
            b"CHGHOST" => {
                let uid = &line.args[0];
                let sid = &uid[..3];
                let server = network.get_server_mut(sid);
                server.get_user_mut(uid).host = line.args[1].to_string();
            }
            b"SJOIN" => {
                //:420 SJOIN 1640815917 #gaynet +MOPnst :@00AAAAAAC 420AAAABC
                let name = line.args[1].to_string();
                let _users = line.args[3].split(' ').map(ToOwned::to_owned);
                let mut channel = Channel::new();

                let modes = modes_from(&line.args[2]);
                let args = line.args[3..].to_vec();
                for (mode, _, arg) in mode_args(modes, args) {
                    channel.modes.insert(mode, arg);
                }

                network.add_channel(name, channel);
            }
            b"ENCAP" => {
                return TS6Handler::handle_line_encap(
                    network,
                    sid,
                    &line.args[0],
                    &line.args[1],
                    &line.args[2..],
                );
            }
            //:420 BAN K * test. 1656888029 31449600 31449600 jess!a0Ob4s0oLV@husky.vpn.lolnerd.net{jess} :moo
            b"BAN" => {
                let btype = line.args[0].chars().next().unwrap();
                let mask = match btype {
                    'K' => format!("{}@{}", line.args[1], line.args[2]),
                    // throw or something instead. only expecting K here
                    _ => "asd".to_string(),
                };
                let since = line.args[3].parse::<u64>().unwrap();
                let duration = line.args[4].parse::<u64>().unwrap();
                let setter = Oper::from(&line.args[6]);
                let reason = line.args[7].to_string();

                let bans = network.bans.entry(btype).or_insert_with(Default::default);
                let ban = Ban::new(reason, since, duration, setter);
                match duration {
                    // this remove works because bans Eq on `mask`
                    0 => bans.remove(&mask),
                    _ => bans.insert(mask, ban),
                };
            }
            //:420 BMASK 1656966926 #test b :test!*@*
            b"BMASK" => {
                let channel = network.get_channel_mut(&line.args[1]);
                let mode = line.args[2].chars().next().unwrap();
                let masks_new = line.args[3].split(' ').map(ToOwned::to_owned);

                let masks = channel
                    .mode_lists
                    .entry(mode)
                    .or_insert_with(Default::default);
                for mask in masks_new {
                    masks.insert(mask);
                }
            }
            _ => {
                return Outcome::Unhandled;
            }
        }

        Outcome::Empty
    }

    fn handle_line_uid(network: &mut Network, uid: &str, line: &Line) -> Outcome {
        match line.command.as_slice() {
            //:420AAAABC QUIT :Quit: Reconnecting
            b"QUIT" => {
                let sid = &uid[..3];
                let server = network.get_server_mut(sid);
                server.del_user(uid);
            }
            //:420AAAABC AWAY :afk
            b"AWAY" => {
                let sid = &uid[..3];
                let server = network.get_server_mut(sid);
                server.get_user_mut(uid).away = line.args.first().map(ToString::to_string);
            }
            //:420AAAABC OPER jess admin
            b"OPER" => {
                let sid = &uid[..3];
                let server = network.get_server_mut(sid);
                server.get_user_mut(uid).oper = Some(line.args[0].to_string());
            }
            //:420AAAABG MODE 420AAAABG :+p-z
            b"MODE" => {
                let uid = &line.args[0];
                let sid = &uid[..3];
                let server = network.get_server_mut(sid);
                let user = server.get_user_mut(uid);

                for (mode, remove) in modes_from(&line.args[1]) {
                    if remove {
                        user.modes.remove(&mode);
                    } else {
                        user.modes.insert(mode);
                    }
                }

                if user.oper.is_some() && !user.modes.contains(&'o') {
                    /* something (hopefully this mode change) caused this user to lose +o,
                    so they're no longer opered */
                    user.oper = None;
                }
            }
            //:420AAAABG TMODE 1656966926 #test -m+mi-i
            b"TMODE" => {
                let channel = network.get_channel_mut(&line.args[1]);
                let modes = modes_from(&line.args[2]);

                for (mode, remove, arg) in mode_args(modes, line.args[3..].to_vec()) {
                    if remove {
                        channel.modes.remove(&mode);
                    } else {
                        channel.modes.insert(mode, arg);
                    }
                }
            }
            _ => {
                return Outcome::Unhandled;
            }
        }

        Outcome::Empty
    }
}

impl Handler for TS6Handler {
    fn get_burst<'a>(
        &self,
        network: &Network,
        password: &'a str,
    ) -> Result<Vec<String>, &'static str> {
        let now = SystemTime::now();

        Ok(vec![
            format!("PASS {} TS 6 :{}", password, network.me.sid),
            "CAPAB :BAN CHW CLUSTER ECHO ENCAP EOPMOD EUID EX IE KLN KNOCK MLOCK QS RSFNC SAVE SERVICES TB UNKLN".to_string(),
            format!(
                "SERVER {} 1 :{}",
                network.me.name, network.me.description
            ),
            format!("SVINFO 6 6 0 {}", now.duration_since(SystemTime::UNIX_EPOCH).map_err(|_e| "GRAN PROBLEMA DE TIEMPO")?.as_secs()),
        ])
    }

    fn handle(&mut self, network: &mut Network, line: &Line) -> Outcome {
        match &line.source {
            None => self.handle_line_none(network, line),
            // lines sourced from a server
            Some(sid) if sid.len() == 3 => TS6Handler::handle_line_sid(network, sid, line),
            // lines sourced from a user
            Some(uid) if uid.len() == 9 => TS6Handler::handle_line_uid(network, uid, line),
            // no idea mate
            _ => Outcome::Unhandled,
        }
    }
}
