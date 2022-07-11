use crate::topic::Topic;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;

#[derive(Default)]
pub struct Channel {
    pub topic: Option<Topic>,
    pub modes: HashMap<char, Option<String>>,
    pub mode_lists: HashMap<char, LinkedHashSet<String>>,
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }
}
