use std::collections::VecDeque;
use std::str::from_utf8;

pub struct Line<'a> {
    pub source: Option<&'a str>,
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl<'a> Line<'a> {
    pub fn from(line: &'a [u8]) -> Self {
        let (mut source, mut line) = line.split_at(match line.first() {
            Some(b':') => line.iter().position(|&c| c == b' ').unwrap(),
            _ => 0,
        });

        if !source.is_empty() {
            source = &source[1..];
            line = &line[1..];
        }

        let mut args: VecDeque<&[u8]> = VecDeque::new();
        while !line.is_empty() {
            let arg_end = match line.first() {
                Some(b':') => {
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
            source: from_utf8(source).ok(),
            command: from_utf8(args.pop_front().unwrap()).unwrap(),
            args: args.iter().map(|&a| from_utf8(a).unwrap()).collect(),
        }
    }
}
