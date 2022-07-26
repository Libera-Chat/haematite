#![deny(clippy::pedantic)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::debug_assert_with_mut_call)]
#![deny(clippy::equatable_if_let)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::same_name_method)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::shadow_unrelated)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_arguments)]

pub mod ban;
pub mod channel;
pub mod config;
pub mod hostmask;
pub mod network;
pub mod oper;
pub mod server;
pub mod topic;
pub mod user;
