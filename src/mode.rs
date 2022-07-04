use std::collections::HashMap;

pub fn modes_from(mode: &str) -> impl Iterator<Item = (char, bool)> {
    let mut remove = false;
    let mut result: HashMap<char, bool> = HashMap::new();

    for mode_char in mode.chars() {
        if mode_char == '+' {
            remove = false;
        } else if mode_char == '-' {
            remove = true;
        } else {
            result.insert(mode_char, remove);
        }
    }

    result.into_iter()
}
