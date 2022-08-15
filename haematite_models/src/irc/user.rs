use serde::{Serialize, Serializer};
use std::collections::HashSet;

#[derive(Default, Serialize)]
pub struct User {
    pub nick: String,
    pub user: String,
    pub host: String,
    pub real: String,
    pub account: Option<String>,
    pub ip: Option<String>,
    pub rdns: Option<String>,
    pub server: String,

    pub channels: HashSet<String>,
    pub modes: HashSet<char>,
    pub oper: Option<String>,
    pub away: Option<String>,
}

pub enum Action<T> {
    Add(T),
    Remove,
}

pub enum Diff {
    Nick(String),
    User(String),
    Host(String),
    Away(Option<String>),
    Oper(Option<String>),
    Mode(char, Action<()>),
}

impl User {
    pub fn new(
        nick: String,
        user: String,
        host: String,
        real: String,
        account: Option<String>,
        ip: Option<String>,
        rdns: Option<String>,
        server: String,
    ) -> Self {
        User {
            nick,
            user,
            host,
            real,
            account,
            ip,
            rdns,
            server,
            ..Self::default()
        }
    }

    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<String, S::Error>
    where
        S: Serializer,
    {
        let name = match diff {
            Diff::Nick(nick) => {
                self.nick = nick;
                self.nick.serialize(ser)?;
                "nick"
            }
            Diff::User(user) => {
                self.user = user;
                self.user.serialize(ser)?;
                "user"
            }
            Diff::Host(host) => {
                self.host = host;
                self.host.serialize(ser)?;
                "host"
            }
            Diff::Mode(char, action) => {
                match action {
                    Action::Add(_) => self.modes.insert(char),
                    Action::Remove => self.modes.remove(&char),
                };
                self.modes.serialize(ser)?;
                "modes"
            }
            Diff::Oper(oper) => {
                self.oper = oper;
                self.oper.serialize(ser)?;
                "oper"
            }
            Diff::Away(away) => {
                self.away = away;
                self.away.serialize(ser)?;
                "away"
            }
        };
        Ok(name.to_owned())
    }
}
