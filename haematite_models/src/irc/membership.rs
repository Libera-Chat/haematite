use serde::Serialize;
use std::collections::HashSet;

#[derive(Default, Serialize)]
pub struct Membership {
    pub status: HashSet<char>,
}
