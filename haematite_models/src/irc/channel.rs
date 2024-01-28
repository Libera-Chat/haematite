use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::Serialize;

use super::membership::Membership;
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
    pub timestamp: u64,
}
