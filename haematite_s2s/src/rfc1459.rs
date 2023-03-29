use std::collections::VecDeque;

use crate::line::{Error, Line};
use crate::util::TakeWord as _;

impl Line {
    pub fn try_from_rfc1459(mut line: &[u8]) -> Result<Line, Error> {
        let source = match line.first() {
            Some(b':') => Some(line.take_word()[1..].to_vec()),
            _ => None,
        };

        let mut args: VecDeque<Vec<u8>> = VecDeque::new();
        loop {
            let arg = match line.first() {
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
            command: args.pop_front().ok_or(Error::MissingCommand)?,
            args: args.into(),
        })
    }
}
