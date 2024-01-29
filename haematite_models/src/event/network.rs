use chrono::NaiveDateTime;
use serde::Serialize;

use crate::irc::oper::Oper;

#[derive(Serialize)]
pub struct AddBan<'a> {
    pub mask: &'a str,
    pub setter: &'a Oper,
    pub reason: &'a str,
    pub since: &'a NaiveDateTime,
    pub expires: &'a NaiveDateTime,
}
