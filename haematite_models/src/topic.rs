use crate::hostmask::Hostmask;
use chrono::NaiveDateTime;

pub enum Setter {
    Hostmask(Hostmask),
    Nickname(String),
}

pub struct Topic {
    pub text: String,
    pub since: NaiveDateTime,
    pub setter: Setter,
}
