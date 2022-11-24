#[derive(Debug)]
pub enum Error {
    MissingCommand,
}

#[derive(Debug)]
pub struct Line {
    pub source: Option<Vec<u8>>,
    pub command: Vec<u8>,
    pub args: Vec<Vec<u8>>,
}
