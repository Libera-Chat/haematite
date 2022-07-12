use crate::hostmask::Hostmask;

pub struct Oper {
    pub name: String,
    pub hostmask: Option<Hostmask>,
}
