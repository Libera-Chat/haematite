use regex::Regex;

pub struct Hostmask {
    _nick: String,
    _user: String,
    _host: String,
}

impl Hostmask {
    pub fn from(hostmask: &str) -> Result<Self, &'static str> {
        // todo: precompile
        let regex = Regex::new(r"^([^!]+)!([^@]{1,10})@(\S+)$").unwrap();
        match regex.captures(hostmask) {
            Some(hostmask) => Ok(Self {
                _nick: hostmask.get(1).unwrap().as_str().to_string(),
                _user: hostmask.get(2).unwrap().as_str().to_string(),
                _host: hostmask.get(3).unwrap().as_str().to_string(),
            }),
            None => Err("invalid hostmask"),
        }
    }
}
