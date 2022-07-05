pub fn modes_from(mode: &str) -> impl Iterator<Item = (char, bool)> {
    let mut result = Vec::new();
    let mut remove = false;

    for mode_char in mode.chars() {
        if mode_char == '+' {
            remove = false;
        } else if mode_char == '-' {
            remove = true;
        } else {
            result.push((mode_char, remove));
        }
    }

    result.into_iter()
}
