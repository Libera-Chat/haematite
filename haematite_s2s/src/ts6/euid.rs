use haematite_events::EventStore;
use haematite_models::irc::network::Network;
use haematite_models::irc::user::User;

use crate::handler::{Error, Outcome};
use crate::line::Line;
use crate::util::mode::split_chars;
use crate::util::{DecodeHybrid as _, NoneOr, TrueOr};

//:420 EUID jess 1 1706456497 +QZaioswz MRuDr7FpIS husky.vpn.lolnerd.net fd84:9d71:8b8:1::1 420AAAAAB * * :big meow
pub fn handle<E: EventStore>(
    event_store: &mut E,
    network: &mut Network,
    line: &Line,
) -> Result<Outcome, Error> {
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

    let mut user = User {
        nick,
        user,
        host,
        real,
        account,
        ip,
        rdns,
        server: sid.clone(),
        ..User::default()
    };

    for (mode, _) in split_chars(&line.args[3].decode()) {
        user.modes.insert(mode);
    }

    let modes = &line.args[3];
    event_store.store(
        "user.connect",
        haematite_models::event::user::Connected {
            uid: &uid,
            nick: &user.nick,
            user: &user.user,
            real: &user.real,
            host: &user.host,
            ip: &user.ip,
            rdns: &user.rdns,
            account: &user.account,
            tls: modes.contains(&b'z'),
        },
    )?;

    network
        .users
        .insert(uid.clone(), user)
        .none_or(Error::InvalidState)?;

    network
        .servers
        .get_mut(&sid)
        .ok_or(Error::InvalidState)?
        .users
        .insert(uid)
        .true_or(Error::InvalidState)?;

    Ok(Outcome::Handled)
}
