use std::collections::HashSet;

use serde::Serialize;

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
    pub certfp: Option<String>,
}

impl User {
    #[must_use]
    pub fn hostmask(&self) -> String {
        format!("{}!{}@{}", self.nick, self.user, self.host)
    }
}
