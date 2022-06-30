use std::collections::VecDeque;

pub struct Line<'a> {
    pub source: Option<&'a str>,
    pub command: &'a str,
    pub args: Vec<&'a str>,
}

impl Line {
    pub fn from(line_full: String) -> Self {
        let mut offset = 0;

        let source = match line_full.chars().next() {
            Some(':') => {
                offset = line_full.find(' ').unwrap() + 1;
                Some(&line_full[0..offset - 1])
            }
            _ => None,
        };

        let mut line = &line_full[offset..];
        let trailing = match line.find(" :") {
            Some(i) => {
                let out = Some(&line[i + 2..]);
                line = &line[..i];
                out
            }
            _ => None,
        };

        let mut args: VecDeque<&str> = line.split(' ').collect();
        match trailing {
            Some(s) => args.push_back(s),
            None => {}
        };

        Line {
            source: source,
            command: args.pop_front().unwrap(),
            args: args.into(),
        }
    }
}
