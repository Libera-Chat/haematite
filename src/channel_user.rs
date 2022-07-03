use std::collections::HashSet;

#[derive(Default)]
pub struct ChannelUser {
    modes: HashSet<char>,
}

impl ChannelUser {
    pub fn new() -> Self {
        ChannelUser {
            ..Default::default()
        }
    }

    pub fn add_mode(mut self, mode: char) -> bool {
        self.modes.insert(mode)
    }
    pub fn del_mode(mut self, mode: char) -> bool {
        self.modes.remove(&mode)
    }
}
