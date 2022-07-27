use haematite_models::irc::network::Network;
use haematite_models::irc::user::User;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::DecodeHybrid as _;

use super::util::state::add_user;

pub fn handle(network: &mut Network, line: &Line) -> Result<Outcome, Error> {
    Line::assert_arg_count(line, 11)?;

    let sid = line.source.as_ref().ok_or(Error::MissingSource)?.decode();
    let uid = line.args[7].decode();

    let nick = line.args[0].decode();
    let user = line.args[4].decode();
    let host = line.args[5].decode();
    let real = line.args[10].decode();
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

    let mut user = User::new(nick, user, host, real, account, ip, rdns, sid);

    for (mode, _) in split_chars(&line.args[3].decode()) {
        user.modes.insert(mode);
    }

    add_user(network, uid, user)?;

    Ok(Outcome::Empty)
}
