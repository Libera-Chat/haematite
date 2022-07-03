use std::collections::VecDeque;
use std::str::from_utf8;

#[derive(Debug)]
pub struct Line<'a> {
    pub source: Option<&'a str>,
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Line<'a> {
    pub fn from(mut line: &'a [u8]) -> Self {
        let source = match line.first() {
            Some(b':') => {
                // find next space
                let end = line.iter().position(|&c| c == b' ').unwrap();
                // grab out source, sans preceding ":", sans trailing space
                let source = &line[1..end];
                // drop space after source from mutable line
                line = &line[end + 1..];
                Some(from_utf8(source).unwrap())
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

        Line {
            source,
            command: from_utf8(args.pop_front().unwrap()).unwrap(),
            args: args.iter().map(|&a| from_utf8(a).unwrap()).collect(),
        }
    }
}
