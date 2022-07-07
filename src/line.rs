use std::collections::VecDeque;
use std::str::from_utf8;

use crate::util;

#[derive(Debug)]
pub enum ParseError {
    MissingSpace,
    MissingCommand,
    SourceDecode,
    ArgDecode(usize),
}

#[derive(Debug)]
pub struct Line {
    pub source: Option<String>,
    pub command: Vec<u8>,
    pub args: Vec<String>,
}

impl Line {
    pub fn from(line: &Vec<u8>) -> Result<Self, ParseError> {
        let mut line = line.iter().peekable();

        let source = match line.peek() {
            Some(b':') => line.take_word().ok_or(ParseError::MissingSpace)?,
            _ => None,
        };

        let mut args: VecDeque<&[u8]> = VecDeque::new();
        loop {
            let arg = match line.peek() {
                Some(b':') => line.collect::<Vec<_>>(),
                Some(_) => line.take_word(),
                None => break,
            };
            args.push_back(arg);
        }

        Ok(Line {
            source,
            command: args.pop_front().ok_or(ParseError::MissingCommand)?.to_vec(),
            args: args
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    from_utf8(a)
                        .map(ToString::to_string)
                        .map_err(|_e| ParseError::ArgDecode(i))
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
