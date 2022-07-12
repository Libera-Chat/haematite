use std::collections::HashMap;

use crate::handler::Outcome;
use crate::line::Line;
use crate::mode::modes_from;
use crate::network::Network;
use crate::user::User;
use crate::util::{DecodeHybrid as _, NoneOr as _, TrueOr as _};

use super::TS6Handler;

impl TS6Handler {
    pub fn handle_euid(network: &mut Network, line: &Line) -> Result<Outcome, &'static str> {
        let sid = line.source.as_ref().ok_or("missing source")?;
        let uid = line.args[7].clone();

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

        let mut user = User::new(nickname, username, realname, account, ip, rdns, host);

        for (mode, _) in modes_from(&line.args[3].decode()) {
            user.modes.insert(mode);
        }

        network
            .users
            .insert(uid.clone(), user)
            .none_or("overwritten uid")?;

        network
            .user_channels
            .insert(uid.clone(), HashMap::default())
            .none_or("overwritten uid")?;

        network
            .user_server
            .insert(uid.clone(), sid.clone())
            .none_or("overwriten uid")?;
        network
            .server_users
            .get_mut(sid)
            .ok_or("unknown sid")?
            .insert(uid)
            .true_or("overwritten uid")?;

        Ok(Outcome::Empty)
    }
}
