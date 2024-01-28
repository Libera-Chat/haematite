use serde::Serialize;

#[derive(Serialize)]
pub struct Hostmask {
    pub nick: String,
    pub user: String,
    pub host: String,
}

pub enum Error {
    InvalidFormat,
}

enum Stage {
    Nick,
    User,
    Host,
}

impl TryFrom<&str> for Hostmask {
    type Error = Error;

    fn try_from(hostmask: &str) -> Result<Self, Self::Error> {
        let mut result = Self {
            nick: String::new(),
            user: String::new(),
            host: String::new(),
        };
        let mut stage = Stage::Nick;

        for char in hostmask.chars() {
            match stage {
                Stage::Nick => {
                    if char == '!' {
                        stage = Stage::User;
                    } else {
                        result.nick.push(char);
                    }
                }
                Stage::User => {
                    if char == '@' {
                        stage = Stage::Host;
                    } else {
                        result.user.push(char);
                    }
                }
                Stage::Host => result.host.push(char),
            }
        }

        if result.host.is_empty() {
            Err(Error::InvalidFormat)
        } else {
            Ok(result)
        }
    }
}
