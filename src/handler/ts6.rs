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
use crate::util::decode_hybrid;

fn mode_args<'a>(
    modes: impl Iterator<Item = (char, bool)>,
    mut args: impl Iterator<Item = &'a Vec<u8>>,
) -> impl Iterator<Item = (char, bool, Option<&'a Vec<u8>>)> {
    let mut out = Vec::new();

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
    uplink: Option<[u8; 3]>,
}

impl TS6Handler {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_line_none(&mut self, network: &mut Network, line: &Line) -> Outcome {
        match line.command.as_slice() {
            b"PASS" => self.uplink = Some(line.args[3].as_slice().try_into().unwrap()),
            b"SERVER" => {
                let sid = self.uplink.take().unwrap();
                network.servers.insert(
                    sid,
                    Server {
                        sid: decode_hybrid(&sid),
                        name: decode_hybrid(&line.args[0]),
                        description: decode_hybrid(&line.args[2]),
                        ..Server::default()
                    },
                );
            }
            b"PING" => {
                return Outcome::Response(vec![format!(
                    ":{} PONG {} {}",
                    network.me.sid,
                    network.me.name,
                    decode_hybrid(&line.args[0]),
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
        _sid: [u8; 3],
        _target: &[u8],
        command: &[u8],
        args: &[Vec<u8>],
    ) -> Outcome {
        match command {
            //:00A ENCAP * SU :420AAAABF
            //:00A ENCAP * SU 420AAAABF :jess
            b"SU" => {
                let uid = args[0].as_slice();
                let server = network.servers.get_mut(&uid[..3]).unwrap();
                server.get_user_mut(uid).account = args.get(1).map(|a| decode_hybrid(a));
            }
            _ => {
                return Outcome::Unhandled;
            }
        }
        Outcome::Empty
    }

    fn handle_line_sid(network: &mut Network, src_sid: [u8; 3], line: &Line) -> Outcome {
        match line.command.as_slice() {
            b"SID" => {
                let sid = line.args[2].clone().try_into().unwrap();
                network.servers.insert(
                    sid,
                    Server {
                        sid: decode_hybrid(&sid),
                        name: decode_hybrid(&line.args[0]),
                        description: decode_hybrid(&line.args[3]),
                        ..Server::default()
                    },
                );
            }
            b"SQUIT" => {
                let sid = &line.args[0];
                network.servers.remove(sid.as_slice());
            }
            //:420 EUID jess 1 1656880345 +QZaioswz a0Ob4s0oLV test. fd84:9d71:8b8:1::1 420AAAABD husky.vpn.lolnerd.net jess :big meow
            b"EUID" => {
                let uid = line.args[7].clone();
                let nickname = decode_hybrid(&line.args[0]);
                let username = decode_hybrid(&line.args[4]);
                let realname = decode_hybrid(&line.args[10]);
                let account = match line.args[9].as_slice() {
                    b"*" => None,
                    account => Some(decode_hybrid(account)),
                };
                let ip = match line.args[6].as_slice() {
                    b"0" => None,
                    ip => Some(decode_hybrid(ip)),
                };
                let rdns = match line.args[8].as_slice() {
                    b"*" => None,
                    rdns => Some(decode_hybrid(rdns)),
                };
                let host = decode_hybrid(&line.args[5]);

                let server = network.servers.get_mut(&src_sid).unwrap();
                let mut user = User::new(nickname, username, realname, account, ip, rdns, host);

                for (mode, _) in modes_from(&decode_hybrid(&line.args[3])) {
                    user.modes.insert(mode);
                }

                server.add_user(uid, user);
            }
            //:00A CHGHOST 420AAAABD husky.vpn.lolnerd.net
            b"CHGHOST" => {
                let uid = &line.args[0];
                let sid = &uid[..3];
                let server = network.servers.get_mut(sid).unwrap();
                server.get_user_mut(uid).host = decode_hybrid(&line.args[1]);
            }
            b"SJOIN" => {
                //:420 SJOIN 1640815917 #gaynet +MOPnst :@00AAAAAAC 420AAAABC
                let name = decode_hybrid(&line.args[1]);
                let _users = decode_hybrid(&line.args[3]).split(' ');
                let mut channel = Channel::new();

                let modes = modes_from(&decode_hybrid(&line.args[2]));
                let args = line.args[3..].iter();
                for (mode, _, arg) in mode_args(modes, args) {
                    channel.modes.insert(mode, arg.map(|a| decode_hybrid(a)));
                }

                network.add_channel(name, channel);
            }
            b"ENCAP" => {
                return TS6Handler::handle_line_encap(
                    network,
                    src_sid,
                    &line.args[0],
                    &line.args[1],
                    line.args[2..].iter().as_slice(),
                );
            }
            //:420 BAN K * test. 1656888029 31449600 31449600 jess!a0Ob4s0oLV@husky.vpn.lolnerd.net{jess} :moo
            b"BAN" => {
                let btype = line.args[0][0] as char;
                let mask = match btype {
                    'K' => format!(
                        "{}@{}",
                        decode_hybrid(&line.args[1]),
                        decode_hybrid(&line.args[2])
                    ),
                    // throw or something instead. only expecting K here
                    _ => "asd".to_string(),
                };
                let since = decode_hybrid(&line.args[3]).parse::<u64>().unwrap();
                let duration = decode_hybrid(&line.args[4]).parse::<u64>().unwrap();
                let setter = Oper::from(&decode_hybrid(&line.args[6]));
                let reason = decode_hybrid(&line.args[7]);

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
                let channel = network.get_channel_mut(&decode_hybrid(&line.args[1]));
                let mode = line.args[2][0] as char;
                let masks_new = line.args[3].split(|c| c == &b' ');

                let masks = channel
                    .mode_lists
                    .entry(mode)
                    .or_insert_with(Default::default);
                for mask in masks_new {
                    masks.insert(decode_hybrid(mask));
                }
            }
            _ => {
                return Outcome::Unhandled;
            }
        }

        Outcome::Empty
    }

    fn handle_line_uid(network: &mut Network, src_uid: &[u8], line: &Line) -> Outcome {
        match line.command.as_slice() {
            //:420AAAABC QUIT :Quit: Reconnecting
            b"QUIT" => {
                let sid = &src_uid[..3];
                let server = network.servers.get_mut(sid).unwrap();
                server.del_user(src_uid);
            }
            //:420AAAABC AWAY :afk
            b"AWAY" => {
                let sid = &src_uid[..3];
                let server = network.servers.get_mut(sid).unwrap();
                server.get_user_mut(src_uid).away = line.args.get(0).map(|a| decode_hybrid(a));
            }
            //:420AAAABC OPER jess admin
            b"OPER" => {
                let sid = &src_uid[..3];
                let server = network.servers.get_mut(sid).unwrap();
                server.get_user_mut(src_uid).oper = Some(decode_hybrid(&line.args[0]));
            }
            //:420AAAABG MODE 420AAAABG :+p-z
            b"MODE" => {
                let uid = &line.args[0];
                let sid = &uid[..3];
                let server = network.servers.get_mut(sid).unwrap();
                let user = server.get_user_mut(uid);

                for (mode, remove) in modes_from(&decode_hybrid(&line.args[1])) {
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
                let channel = network.get_channel_mut(&decode_hybrid(&line.args[1]));
                let modes = modes_from(&decode_hybrid(&line.args[2]));
                let args = line.args[3..].iter();

                for (mode, remove, arg) in mode_args(modes, args) {
                    if remove {
                        channel.modes.remove(&mode);
                    } else {
                        channel.modes.insert(mode, arg.map(|a| decode_hybrid(a)));
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
            Some(source) => {
                if source.len() == 3 {
                    let mut sid: [u8; 3] = [0; 3];
                    sid.copy_from_slice(source);
                    TS6Handler::handle_line_sid(network, sid, line)
                } else {
                    TS6Handler::handle_line_uid(network, source, line)
                }
            }
        }
    }
}
