use chrono::naive::serde::ts_seconds;
use chrono::NaiveDateTime;
use serde::Serialize;

use super::oper::Oper;

#[derive(Serialize)]
pub struct Ban {
    pub expires: NaiveDateTime,
    pub reason: String,
    pub setter: Oper,
    #[serde(with = "ts_seconds")]
    pub since: NaiveDateTime,
}
