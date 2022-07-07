use std::collections::HashMap;

#[derive(Default)]
pub struct Channel {
    pub modes: HashMap<char, Option<String>>,
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }
}
