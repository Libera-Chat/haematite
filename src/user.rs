#[derive(Default)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub realname: String,
    pub account: Option<String>,
    pub ip: Option<String>,
    pub realhost: String,
    pub showhost: String,

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
        realhost: String,
        showhost: String,
    ) -> Self {
        User {
            nickname,
            username,
            realname,
            account,
            ip,
            realhost,
            showhost,
            ..Default::default()
        }
    }
}
