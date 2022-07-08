use std::collections::VecDeque;
use std::str::from_utf8;

use crate::util::TakeWord as _;

#[derive(Debug)]
pub enum ParseError {
    MissingCommand,
    ArgDecode(usize),
}

#[derive(Debug)]
pub struct Line {
    pub source: Option<Vec<u8>>,
    pub command: Vec<u8>,
    pub args: Vec<String>,
}

impl Line {
    pub fn from(mut line: &[u8]) -> Result<Self, ParseError> {
        let source = match line.get(0) {
            Some(b':') => Some(line.take_word()[1..].to_vec()),
            _ => None,
        };

        let mut args: VecDeque<&[u8]> = VecDeque::new();
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
