use regex::Regex;

use crate::hostmask::Hostmask;

pub struct Oper {
    _name: String,
    _hostmask: Option<Hostmask>,
}

impl TryFrom<&str> for Oper {
    type Error = &'static str;

    fn try_from(mut oper: &str) -> Result<Self, Self::Error> {
        let oper_regex = Regex::new(r"^([^{]+)\{(\S+)\}$").unwrap();

        let hostmask = match oper_regex.captures(oper) {
            Some(hmatch) => {
                let hostmask = hmatch.get(0).unwrap().as_str();
                oper = hmatch.get(1).unwrap().as_str();
                Some(Hostmask::from(hostmask)?)
            }
            None => None,
        };

        Ok(Oper {
            _name: oper.to_string(),
            _hostmask: hostmask,
        })
    }
}
