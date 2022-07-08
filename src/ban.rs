use std::time::Duration;

use crate::oper::Oper;
use chrono::NaiveDateTime;

pub struct Ban {
    _reason: String,
    _since: NaiveDateTime,
    _duration: Duration,
    _setter: Oper,
}

impl Ban {
    pub fn new(reason: String, since: u64, duration: u64, setter: Oper) -> Self {
        Self {
            _reason: reason,
            _since: NaiveDateTime::from_timestamp(since.try_into().unwrap(), 0),
            _duration: Duration::from_secs(duration),
            _setter: setter,
        }
    }
}
