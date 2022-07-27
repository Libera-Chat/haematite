use serde::Serialize;
use std::collections::HashSet;

#[derive(Default, Serialize)]
pub struct User {
    pub nick: String,
    pub user: String,
    pub host: String,
    pub real: String,
    pub account: Option<String>,
    pub ip: Option<String>,
    pub rdns: Option<String>,
    pub server: String,

    pub channels: HashSet<String>,
    pub modes: HashSet<char>,
    pub oper: Option<String>,
    pub away: Option<String>,
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
            nick,
            user,
            host,
            real,
            account,
            ip,
            rdns,
            server,
            ..Self::default()
        }
    }
}
