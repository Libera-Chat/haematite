use std::collections::HashSet;

use serde::Serialize;

use crate::permission;
use crate::permission::With;

#[derive(Default, Serialize)]
pub struct User {
    pub nick: With<String, permission::user::Nick>,
    pub user: With<String, permission::user::User>,
    pub host: With<String, permission::user::Host>,
    pub real: With<String, permission::user::Real>,
    pub account: With<Option<String>, permission::user::Account>,
    pub ip: With<Option<String>, permission::user::Ip>,
    pub rdns: With<Option<String>, permission::user::Rdns>,
    pub server: With<String, permission::user::Server>,

    pub channels: HashSet<String>,
    pub modes: With<HashSet<char>, permission::user::Modes>,
    pub oper: With<Option<String>, permission::user::Oper>,
    pub away: With<Option<String>, permission::user::Away>,
}

impl User {
    pub fn new(
        nick: String,
        user: String,
        host: String,
        real: String,
        account: Option<String>,
        ip: Option<String>,
        rdns: Option<String>,
        server: String,
    ) -> Self {
        User {
            nick: nick.into(),
            user: user.into(),
            host: host.into(),
            real: real.into(),
            account: account.into(),
            ip: ip.into(),
            rdns: rdns.into(),
            server: server.into(),
            ..Self::default()
        }
    }
}
