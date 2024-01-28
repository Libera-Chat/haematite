pub mod amqp;

use async_trait::async_trait;

#[derive(Debug)]
pub enum Error {
    Amqp(lapin::Error),
}

impl From<lapin::Error> for Error {
    fn from(value: lapin::Error) -> Self {
        Self::Amqp(value)
    }
}

#[async_trait]
pub trait Handler {
    async fn publish(&mut self, topic: &'static str, payload: &[u8]) -> Result<(), Error>;
}
