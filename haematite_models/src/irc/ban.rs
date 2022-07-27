use std::time::Duration;

use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde::Serialize;

use super::oper::Oper;

#[derive(Serialize)]
pub struct Ban {
    pub reason: String,
    #[serde(with = "ts_seconds")]
    pub since: NaiveDateTime,
    pub duration: Duration,
    pub setter: Oper,
}
