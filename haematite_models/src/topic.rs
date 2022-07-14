use crate::hostmask::Hostmask;
use chrono::NaiveDateTime;

pub struct Topic {
    pub text: String,
    pub since: NaiveDateTime,
    pub setter: Hostmask,
}
