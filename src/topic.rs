use crate::hostmask::Hostmask;
use chrono::NaiveDateTime;

pub struct Topic {
    pub text: String,
    pub since: NaiveDateTime,
    pub setter: Hostmask,
}

impl Topic {
    pub fn new(text: String, since: u32, setter: &str) -> Result<Self, &'static str> {
        Ok(Self {
            text,
            since: NaiveDateTime::from_timestamp(i64::from(since), 0),
            setter: Hostmask::from(setter)?,
        })
    }
}
