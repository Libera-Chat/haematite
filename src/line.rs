use std::collections::VecDeque;
use std::str::from_utf8;

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
    pub fn from(mut line: &[u8]) -> Result<Self, ParseError> {
        let source = match line.first() {
            Some(b':') => {
                // find next space
                let end = line
                    .iter()
                    .position(|&c| c == b' ')
                    .ok_or(ParseError::MissingSpace)?;
                // grab out source, sans preceding ":", sans trailing space
                let source = &line[1..end];
                // drop space after source from mutable line
                line = &line[end + 1..];
                Some(
                    from_utf8(source)
                        .map_err(|_| ParseError::SourceDecode)?
                        .to_string(),
                )
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
                line = &line[1..];
            }
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
