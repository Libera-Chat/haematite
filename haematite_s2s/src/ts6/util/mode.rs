use crate::util::mode::{ArgType, Change};

pub fn to_changes(modes: impl Iterator<Item = (char, bool)>) -> Vec<Change> {
    let mut out = Vec::new();

    for (mode, remove) in modes {
        let arg_type = match mode {
            'k' | 'o' | 'v' => ArgType::One,
            'f' | 'j' | 'l' if !remove => ArgType::One,
            'E' | 'b' | 'e' | 'q' => ArgType::Many,
            _ => ArgType::None,
        };
        out.push(Change {
            mode,
            arg_type,
            remove,
        });
    }

    out
}
