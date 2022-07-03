#[derive(Default)]
pub struct User {
    pub nickname: String,
    pub oper: Option<String>,
    pub away: Option<String>,
}

impl User {
    pub fn new(nickname: String) -> Self {
        User {
            nickname,
            ..Default::default()
        }
    }
}
