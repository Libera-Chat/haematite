use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::hostmask::Hostmask;

#[derive(Serialize)]
pub enum Setter {
    #[serde(rename = "hostmask")]
    Hostmask(Hostmask),
    #[serde(rename = "nickname")]
    Nickname(String),
}

#[derive(Serialize)]
pub struct Topic {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub since: NaiveDateTime,
    pub setter: Setter,
}
