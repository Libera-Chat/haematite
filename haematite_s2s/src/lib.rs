#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]

pub mod handler;
mod line;
mod rfc1459;
pub mod ts6;
mod util;

pub use crate::util::DecodeHybrid;
