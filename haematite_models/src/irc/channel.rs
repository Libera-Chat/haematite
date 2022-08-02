use std::collections::HashMap;

use linked_hash_set::LinkedHashSet;
use serde::Serialize;

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

    pub fn update(&mut self, diff: Diff) {
        match diff {
            Diff::Topic(topic) => self.topic = topic,
            Diff::User(uid, diff) => self.users.get_mut(&uid).unwrap().update(diff),
        };
    }
}
