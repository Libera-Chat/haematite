use std::collections::VecDeque;

use crate::util::TakeWord as _;

#[derive(Debug)]
pub enum ParseError {
    MissingCommand,
}

#[derive(Debug)]
pub struct Line {
    pub source: Option<Vec<u8>>,
    pub command: Vec<u8>,
    pub args: Vec<Vec<u8>>,
}

impl Line {
    pub fn from(mut line: &[u8]) -> Result<Self, ParseError> {
        let source = match line.get(0) {
            Some(b':') => Some(line.take_word()[1..].to_vec()),
            _ => None,
        };

        let mut args: VecDeque<Vec<u8>> = VecDeque::new();
        loop {
            let arg = match line.get(0) {
                Some(b':') => {
                    let arg = &line[1..];
                    line = &line[line.len()..];
                    arg
                }
                Some(_) => line.take_word(),
                None => break,
            };
            args.push_back(arg.to_vec());
        }

        Ok(Line {
            source,
            command: args.pop_front().ok_or(ParseError::MissingCommand)?,
            args: args.into(),
        })
    }
}
