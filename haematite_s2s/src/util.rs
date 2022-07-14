use encoding_rs::WINDOWS_1252;
use std::str::from_utf8;

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

pub fn decode_hybrid(data: &[u8]) -> String {
    if let Ok(utf8) = from_utf8(data) {
        utf8.to_string()
    } else {
        let (cow, _encoding_used, _had_errors) = WINDOWS_1252.decode(data);
        cow[..].to_string()
    }
}

pub trait DecodeHybrid {
    fn decode(&self) -> String;
}

impl DecodeHybrid for [u8] {
    fn decode(&self) -> String {
        decode_hybrid(self)
    }
}

impl DecodeHybrid for &[u8] {
    fn decode(&self) -> String {
        decode_hybrid(self)
    }
}

impl DecodeHybrid for Vec<u8> {
    fn decode(&self) -> String {
        decode_hybrid(self)
    }
}

pub trait NoneOr<T> {
    fn none_or<E>(&mut self, error: E) -> Result<(), E>;
}

impl<T> NoneOr<T> for Option<T> {
    fn none_or<E>(&mut self, error: E) -> Result<(), E> {
        match self.take() {
            Some(_) => Err(error),
            None => Ok(()),
        }
    }
}

pub trait FalseOr {
    fn false_or<E>(&self, error: E) -> Result<(), E>;
}

impl FalseOr for bool {
    fn false_or<E>(&self, error: E) -> Result<(), E> {
        if *self {
            Err(error)
        } else {
            Ok(())
        }
    }
}

pub trait TrueOr {
    fn true_or<E>(&self, error: E) -> Result<(), E>;
}

impl TrueOr for bool {
    fn true_or<E>(&self, error: E) -> Result<(), E> {
        if *self {
            Ok(())
        } else {
            Err(error)
        }
    }
}
