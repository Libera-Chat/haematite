use crate::handler::Error;
use async_trait::async_trait;
use lapin::{
    options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties,
};

pub struct Handler {
    chan: Channel,
}

impl Handler {
    pub async fn connect(addr: &str) -> Result<Self, lapin::Error> {
        let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
        let chan = conn.create_channel().await?;
        Ok(Self { chan })
    }
}

#[async_trait]
impl crate::handler::Handler for Handler {
    async fn publish(&mut self, topic: &'static str, payload: &[u8]) -> Result<(), Error> {
        self.chan
            .basic_publish(
                "haematite",
                topic,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }
}
