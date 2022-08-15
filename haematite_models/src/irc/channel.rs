use std::collections::HashMap;

use linked_hash_set::LinkedHashSet;
use serde::{Serialize, Serializer};

use super::topic::Topic;
use crate::irc::membership::{Diff as MembershipDiff, Membership};

#[derive(Default, Serialize)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, LinkedHashSet<String>>,
    pub users: HashMap<String, Membership>,
}

pub enum Diff {
    Topic(Option<Topic>),
    User(String, MembershipDiff),
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<String, S::Error>
    where
        S: Serializer,
    {
        match diff {
            Diff::Topic(topic) => {
                self.topic = topic;
                self.topic.serialize(ser)?;
                Ok("topic".to_owned())
            }
            Diff::User(uid, diff) => {
                let name = self.users.get_mut(&uid).unwrap().update(diff, ser)?;
                Ok(format!("{}/{}", uid, name))
            }
        }
    }
}
