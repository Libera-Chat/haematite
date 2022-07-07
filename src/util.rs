use std::iter::Peekable;
use std::slice::Iter;

trait TakeWord {
    fn take_word(&mut self) -> Option<Vec<u8>>;
}

impl TakeWord for Peekable<Iter<'_, u8>> {
    fn take_word(&mut self) -> Option<Vec<u8>> {
        let space = self.position(|c| c == &b' ').unwrap_or(self.len());
        let word = (0..space)
            .map(|_i| self.next().unwrap().clone())
            .collect::<Vec<u8>>();
        // remove the space
        self.next();
        (!word.is_empty()).then(|| word)
    }
}
