use haematite_models::irc::network::{Action as NetAction, Diff as NetDiff};
use haematite_models::irc::server::{Action as ServAction, Diff as ServDiff};
use haematite_models::irc::user::User;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::DecodeHybrid as _;

pub fn handle(line: &Line) -> Result<Outcome, Error> {
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

    let mut user = User::new(nick, user, host, real, account, ip, rdns, sid.clone());

    for (mode, _) in split_chars(&line.args[3].decode()) {
        user.modes.push(mode);
    }

    Ok(Outcome::State(vec![
        NetDiff::ExternalUser(uid.clone(), NetAction::Add(user)),
        NetDiff::InternalServer(sid, ServDiff::User(uid, ServAction::Add)),
    ]))
}
