pub mod channel;
pub mod network;
pub mod server;
pub mod user;

pub trait GetName {
    fn name();
}
