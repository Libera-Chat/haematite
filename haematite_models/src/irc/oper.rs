use super::hostmask::Hostmask;
use serde::Serialize;

#[derive(Serialize)]
pub struct Oper {
    pub name: String,
    pub hostmask: Option<Hostmask>,
}
