use std::collections::VecDeque;

impl From<serde_json::Error> for super::Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Default)]
pub struct EventStore {
    pub payloads: VecDeque<(&'static str, Vec<u8>)>,
}

impl crate::EventStore for EventStore {
    fn store<S: serde::Serialize>(
        &mut self,
        item_type: &'static str,
        item: S,
    ) -> Result<(), super::Error> {
        let mut buffer = Vec::new();
        item.serialize(&mut serde_json::Serializer::new(&mut buffer))?;
        self.payloads.push_back((item_type, buffer));
        Ok(())
    }
}
