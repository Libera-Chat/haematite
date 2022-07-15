use std::ops::RangeFrom;

#[derive(Debug)]
pub enum Error {
    ExcessArguments(usize, u8),
    InsufficientArguments(usize, u8),
    MissingCommand,
}

#[derive(Debug)]
pub struct Line {
    pub source: Option<Vec<u8>>,
    pub command: Vec<u8>,
    pub args: Vec<Vec<u8>>,
}

pub struct ArgRange {
    minimum: u8,
    maximum: u8,
}

impl From<u8> for ArgRange {
    fn from(other: u8) -> Self {
        Self {
            minimum: other,
            maximum: other,
        }
    }
}

impl From<RangeFrom<u8>> for ArgRange {
    fn from(other: RangeFrom<u8>) -> Self {
        Self {
            minimum: other.start,
            maximum: u8::MAX,
        }
    }
}

impl Line {
    pub fn assert_arg_count(&self, expected: impl Into<ArgRange>) -> Result<(), Error> {
        let actual = self.args.len();
        let expected: ArgRange = expected.into();

        if actual < expected.minimum.into() {
            Err(Error::InsufficientArguments(actual, expected.minimum))
        } else if actual > expected.maximum.into() {
            Err(Error::ExcessArguments(actual, expected.maximum))
        } else {
            Ok(())
        }
    }
}
