use regex::Regex;

pub struct Hostmask {
    pub nick: String,
    pub user: String,
    pub host: String,
}

pub enum Error {
    //TODO: better name
    Bad,
}

impl TryFrom<&str> for Hostmask {
    type Error = Error;

    fn try_from(hostmask: &str) -> Result<Self, Self::Error> {
        //TODO: precompile
        let regex = Regex::new(r"^([^!]+)!([^@]{1,10})@(\S+)$").unwrap();
        match regex.captures(hostmask) {
            Some(hostmask) => Ok(Self {
                nick: hostmask.get(1).unwrap().as_str().to_string(),
                user: hostmask.get(2).unwrap().as_str().to_string(),
                host: hostmask.get(3).unwrap().as_str().to_string(),
            }),
            None => Err(Error::Bad),
        }
    }
}
