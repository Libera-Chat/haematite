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
    from_utf8(data).map_or_else(
        |_| {
            let (cow, _encoding_used, _had_errors) = WINDOWS_1252.decode(data);
            cow[..].to_string()
        },
        std::string::ToString::to_string,
    )
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
