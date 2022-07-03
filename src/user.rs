pub struct User {
    pub nickname: String,
    pub oper: Option<String>,
}

impl User {
    pub fn new(nickname: String) -> Self {
        User {
            nickname,
            oper: None,
        }
    }
}
