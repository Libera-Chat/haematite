pub mod mode;
mod or;
mod str;

pub use self::or::{FalseOr, NoneOr, TrueOr};
pub use self::str::{DecodeHybrid, TakeWord};
