use std::collections::HashSet;

#[derive(Default)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub realname: String,
    pub account: Option<String>,
    pub ip: Option<String>,
    pub rdns: Option<String>,
    pub host: String,

    pub modes: HashSet<char>,
    pub oper: Option<String>,
    pub away: Option<String>,
}

impl User {
    pub fn new(
        nickname: String,
        username: String,
        realname: String,
        account: Option<String>,
        ip: Option<String>,
        rdns: Option<String>,
        host: String,
    ) -> Self {
        User {
            nickname,
            username,
            realname,
            account,
            ip,
            rdns,
            host,
            ..Self::default()
        }
    }
}
