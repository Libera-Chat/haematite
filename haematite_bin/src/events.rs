use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub enum Error {
    Handler(haematite_events::handler::Error),
}

impl From<haematite_events::handler::Error> for Error {
    fn from(value: haematite_events::handler::Error) -> Self {
        Self::Handler(value)
    }
}

pub async fn run<H: haematite_events::handler::Handler + Send>(
    mut handler: H,
    mut rx: Receiver<(&'static str, Vec<u8>)>,
) -> Result<(), Error> {
    while let Some((payload_type, payload)) = rx.recv().await {
        handler.publish(payload_type, &payload).await?;
    }

    handler.finish().await;

    Ok(())
}
