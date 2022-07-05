use regex::Regex;

use crate::hostmask::Hostmask;

pub struct Oper {
    _name: String,
    _hostmask: Option<Hostmask>,
}

impl Oper {
    pub fn from(mut oper: &str) -> Self {
        let hostmask_regex = Regex::new(r"^([^{]+)\{(\S+)\}$").unwrap();
        let hostmask = match hostmask_regex.captures(oper) {
            Some(hmatch) => {
                let hostmask = hmatch.get(0).unwrap().as_str();
                oper = hmatch.get(1).unwrap().as_str();
                Hostmask::from(hostmask)
            }
            None => None,
        };

        Oper {
            _name: oper.to_string(),
            _hostmask: hostmask,
        }
    }
}
