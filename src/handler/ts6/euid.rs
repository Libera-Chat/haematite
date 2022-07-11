use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::user::User;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_euid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid = line.source.as_ref().ok_or("missing source")?.as_slice();
        let uid: [u8; 9] = line.args[7].clone().try_into().map_err(|_| "invalid uid")?;

        let nickname = line.args[0].decode();
        let username = line.args[4].decode();
        let realname = line.args[10].decode();
        let account = match line.args[9].as_slice() {
            b"*" => None,
            account => Some(account.decode()),
        };
        let ip = match line.args[6].as_slice() {
            b"0" => None,
            ip => Some(ip.decode()),
        };
        let rdns = match line.args[8].as_slice() {
            b"*" => None,
            rdns => Some(rdns.decode()),
        };
        let host = line.args[5].decode();

        let server = network.servers.get_mut(sid).unwrap();
        let mut user = User::new(nickname, username, realname, account, ip, rdns, host);

        for (mode, _) in modes_from(&line.args[3].decode()) {
            user.modes.insert(mode);
        }

        server.users.insert(uid, user);

        Ok(Outcome::Empty)
    }
}
