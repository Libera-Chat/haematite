use crate::oper::Oper;
use chrono::NaiveDateTime;
use std::time::Duration;

pub struct Ban {
    pub reason: String,
    pub since: NaiveDateTime,
    pub duration: Duration,
    pub setter: Oper,
}
