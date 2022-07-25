use std::collections::{HashMap, HashSet};

use linked_hash_set::LinkedHashSet;
use serde::Serialize;

use crate::topic::Topic;

#[derive(Default, Serialize)]
pub struct Membership {
    pub status: HashSet<char>,
}

#[derive(Default, Serialize)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, LinkedHashSet<String>>,
    pub users: HashMap<String, Membership>,
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
