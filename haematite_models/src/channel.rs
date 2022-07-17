use crate::topic::Topic;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Membership {
    pub status: HashSet<char>,
}

#[derive(Default)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, LinkedHashSet<String>>,
    pub users: HashMap<Vec<u8>, Membership>,
}

impl Membership {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }
}
