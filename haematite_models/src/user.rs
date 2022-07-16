use crate::permission;
use crate::permission::With;
use std::collections::HashSet;

#[derive(Default)]
pub struct User {
    pub nickname: With<String, permission::UserInfo>,
    pub username: With<String, permission::UserInfo>,
    pub realname: With<String, permission::UserInfo>,
    pub account: With<Option<String>, permission::UserInfo>,
    pub ip: Option<String>,
    pub rdns: Option<String>,
    pub host: With<String, permission::UserInfo>,

    pub modes: HashSet<char>,
    pub oper: Option<String>,
    pub away: With<Option<String>, permission::UserInfo>,
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
            nickname: nickname.into(),
            username: username.into(),
            realname: realname.into(),
            account: account.into(),
            ip,
            rdns,
            host: host.into(),
            ..Self::default()
        }
    }
}
