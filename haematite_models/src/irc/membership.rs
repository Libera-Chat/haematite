use serde::Serialize;
use std::collections::HashSet;

#[derive(Default, Serialize)]
pub struct Membership {
    pub status: HashSet<char>,
}

pub enum Action {
    Add,
    Remove,
}
pub enum Diff {
    Status(char, Action),
}

impl Membership {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, diff: Diff) {
        match diff {
            Diff::Status(char, action) => match action {
                Action::Add => self.status.insert(char),
                Action::Remove => self.status.remove(&char),
            },
        };
    }
}
