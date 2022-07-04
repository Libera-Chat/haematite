use std::collections::VecDeque;
use std::str::from_utf8;

#[derive(Debug)]
pub enum LineError {
    MissingSpace,
    MissingCommand,
    SourceDecode,
    ArgDecode,
}

#[derive(Debug)]
pub struct Line<'a> {
    pub source: Option<&'a str>,
    pub command: &'a [u8],
    pub args: Vec<&'a str>,
}

impl<'a> Line<'a> {
    pub fn from(mut line: &'a [u8]) -> Result<Self, LineError> {
        let source = match line.first() {
            Some(b':') => {
                // find next space
                let end = line
                    .iter()
                    .position(|&c| c == b' ')
                    .ok_or(LineError::MissingSpace)?;
                // grab out source, sans preceding ":", sans trailing space
                let source = &line[1..end];
                // drop space after source from mutable line
                line = &line[end + 1..];
                Some(from_utf8(source).map_err(|_| LineError::SourceDecode)?)
            }
            _ => None,
        };

        let mut args: VecDeque<&[u8]> = VecDeque::new();
        while !line.is_empty() {
            let arg_end = match line.first() {
                Some(b':') => {
                    /* we've got an arg that starts with ":",
                    everything after it is one whole arg */
                    line = &line[1..];
                    line.len()
                }
                _ => line.iter().position(|&c| c == b' ').unwrap_or(line.len()),
            };
            let (arg, remaining) = line.split_at(arg_end);
            line = remaining;
            args.push_back(arg);

            if !line.is_empty() {
                line = &line[1..]
            }
        }

        Ok(Line {
            source,
            command: args.pop_front().ok_or(LineError::MissingCommand)?,
            args: args
                .iter()
                .map(|a| from_utf8(a))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_e| LineError::ArgDecode)?,
        })
    }
}
