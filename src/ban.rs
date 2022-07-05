use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::oper::Oper;
use chrono::NaiveDateTime;

pub struct Ban {
    mask: String,
    _reason: String,
    _since: NaiveDateTime,
    _duration: Duration,
    _setter: Oper,
}

impl Ban {
    pub fn new(mask: String, reason: String, since: u64, duration: u64, setter: Oper) -> Self {
        Self {
            mask,
            _reason: reason,
            _since: NaiveDateTime::from_timestamp(since.try_into().unwrap(), 0),
            _duration: Duration::from_secs(duration),
            _setter: setter,
        }
    }
}

impl PartialEq for Ban {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
    }
}

impl Eq for Ban {}

impl Hash for Ban {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mask.hash(state)
    }
}
