use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::user::User;
use crate::util::DecodeHybrid as _;

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_euid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid: &[u8; 3] = line
            .source
            .as_ref()
            .ok_or("missing source")?
            .as_slice()
            .try_into()
            .map_err(|_| "invalid sid")?;
        let args: &[Vec<u8>; 11] = line
            .args
            .as_slice()
            .try_into()
            .map_err(|_| "missing argument")?;

        let uid: [u8; 9] = args[7].clone().try_into().map_err(|_| "invalid uid")?;
        let nickname = args[0].decode();
        let username = args[4].decode();
        let realname = args[10].decode();
        let account = match args[9].as_slice() {
            b"*" => None,
            account => Some(account.decode()),
        };
        let ip = match args[6].as_slice() {
            b"0" => None,
            ip => Some(ip.decode()),
        };
        let rdns = match args[8].as_slice() {
            b"*" => None,
            rdns => Some(rdns.decode()),
        };
        let host = args[5].decode();

        let server = network.servers.get_mut(sid).unwrap();
        let mut user = User::new(nickname, username, realname, account, ip, rdns, host);

        for (mode, _) in modes_from(&args[3].decode()) {
            user.modes.insert(mode);
        }

        server.users.insert(uid, user);

        Ok(Outcome::Empty)
    }
}
