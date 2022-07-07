pub trait TakeWord<'a> {
    fn take_word(&mut self) -> &'a [u8];
}

impl<'a> TakeWord<'a> for &'a [u8] {
    fn take_word(&mut self) -> &'a [u8] {
        if let Some(i) = self.iter().position(|c| c == &b' ') {
            let word = &self[..i];
            *self = &self[i + 1..];
            word
        } else {
            let word = &self[..];
            *self = &self[self.len()..];
            word
        }
    }
}
