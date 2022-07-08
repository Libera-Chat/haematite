use regex::Regex;

pub struct Hostmask {
    _nick: String,
    _user: String,
    _host: String,
}

impl Hostmask {
    pub fn from(hostmask: &str) -> Option<Self> {
        // todo: precompile
        let regex = Regex::new(r"^([^!]+)!([^@]{1,10})@(\S+)$").unwrap();
        regex.captures(hostmask).map(|hostmask| Self {
            _nick: hostmask.get(0).unwrap().as_str().to_string(),
            _user: hostmask.get(1).unwrap().as_str().to_string(),
            _host: hostmask.get(2).unwrap().as_str().to_string(),
        })
    }
}
