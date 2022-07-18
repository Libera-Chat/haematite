use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use linked_hash_map::LinkedHashMap;

use crate::topic::Topic;

#[derive(Debug)]
pub struct ModeMetadata {
    pub since: NaiveDateTime,
    pub setter: String,
}

#[derive(Default)]
pub struct Membership {
    pub status: HashSet<char>,
}

#[derive(Default)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, LinkedHashMap<String, Option<ModeMetadata>>>,
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
