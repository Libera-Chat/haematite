use super::error::Error;
use serde::{Serialize, Serializer};

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

    pub channels: Vec<String>,
    pub modes: Vec<char>,
    pub oper: Option<String>,
    pub away: Option<String>,
}

pub enum Action {
    Add,
    Remove,
}

pub enum Diff {
    Nick(String),
    User(String),
    Host(String),
    Account(Option<String>),
    Channel(String, Action),
    Mode(char, Action),
    Oper(Option<String>),
    Away(Option<String>),
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

    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(String, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::Nick(nick) => {
                self.nick = nick;
                ("nick".to_owned(), self.nick.serialize(ser)?)
            }
            Diff::User(user) => {
                self.user = user;
                ("user".to_owned(), self.user.serialize(ser)?)
            }
            Diff::Host(host) => {
                self.host = host;
                ("host".to_owned(), self.host.serialize(ser)?)
            }
            Diff::Account(account) => {
                self.account = account;
                ("account".to_owned(), self.account.serialize(ser)?)
            }
            Diff::Mode(char, action) => {
                let (index, value) = match action {
                    Action::Add => {
                        self.modes.push(char);
                        (self.modes.len() - 1, char.serialize(ser)?)
                    }
                    Action::Remove => {
                        let index = self
                            .modes
                            .iter()
                            .position(|&c| c == char)
                            .ok_or(Error::UnknownMode)?;
                        self.modes.remove(index);
                        (index, ser.serialize_none()?)
                    }
                };
                (format!("modes/{}", index), value)
            }
            Diff::Oper(oper) => {
                self.oper = oper;
                ("oper".to_owned(), self.oper.serialize(ser)?)
            }
            Diff::Away(away) => {
                self.away = away;
                ("away".to_owned(), self.away.serialize(ser)?)
            }
            Diff::Channel(name, action) => {
                let (index, value) = match action {
                    Action::Add => {
                        let value = name.serialize(ser)?;
                        self.channels.push(name);
                        (self.channels.len() - 1, value)
                    }
                    Action::Remove => {
                        let index = self
                            .channels
                            .iter()
                            .position(|c| c == &name)
                            .ok_or(Error::UnknownChannel)?;
                        self.channels.remove(index);
                        (index, ser.serialize_none()?)
                    }
                };
                (format!("channels/{}", index), value)
            }
        })
    }
}
