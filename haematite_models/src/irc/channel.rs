use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

use super::error::Error;
use super::membership::{Diff as MembershipDiff, Membership};
use super::topic::Topic;

#[derive(Debug, Serialize)]
pub struct ModeMetadata {
    pub since: NaiveDateTime,
    pub setter: String,
}

#[derive(Default, Serialize)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, HashMap<String, Option<ModeMetadata>>>,
    pub users: HashMap<String, Membership>,
}

pub enum Action<T> {
    Add(T),
    Remove,
}

pub enum Diff {
    Topic(Option<Topic>),
    Mode(char, Action<Option<String>>),
    ModeList(char, String, Action<Option<ModeMetadata>>),
    InternalUser(String, MembershipDiff),
    ExternalUser(String, Action<Membership>),
}

impl Channel {
    pub fn new(mode_lists: HashSet<char>) -> Self {
        Self {
            mode_lists: mode_lists
                .into_iter()
                .map(|c| (c, HashMap::new()))
                .collect(),
            ..Self::default()
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if the presented diff is not applicable to the
    /// current network state, or if the result data cannot be serialized.
    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(String, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::Topic(topic) => {
                self.topic = topic;
                ("topic".to_owned(), self.topic.serialize(ser)?)
            }
            Diff::Mode(mode, action) => {
                let value = match action {
                    Action::Add(arg) => {
                        let value = arg.serialize(ser)?;
                        self.modes.insert(mode, arg);
                        value
                    }
                    Action::Remove => {
                        self.modes.remove(&mode);
                        ser.serialize_none()?
                    }
                };
                (format!("modes/{}", mode), value)
            }
            Diff::ModeList(mode, mask, action) => {
                let path = format!("mode_lists/{}/{}", mode, mask);
                let map = self.mode_lists.get_mut(&mode).ok_or(Error::UnknownMode)?;
                let value = match action {
                    Action::Add(arg) => {
                        let value = arg.serialize(ser)?;
                        map.insert(mask, arg);
                        value
                    }
                    Action::Remove => {
                        map.remove(&mask);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }
            Diff::InternalUser(uid, diff) => {
                let (path, value) = self
                    .users
                    .get_mut(&uid)
                    .ok_or(Error::UnknownUser)?
                    .update(diff, ser)?;
                (format!("users/{}/{}", uid, path), value)
            }
            Diff::ExternalUser(uid, action) => {
                let path = format!("users/{}", uid);
                let value = match action {
                    Action::Add(membership) => {
                        let value = membership.serialize(ser)?;
                        self.users.insert(uid, membership);
                        value
                    }
                    Action::Remove => {
                        self.users.remove(&uid);
                        ser.serialize_none()?
                    }
                };
                (path, value)
            }
        })
    }
}
