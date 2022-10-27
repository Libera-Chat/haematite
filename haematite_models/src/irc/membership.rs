use super::error::Error;
use crate::meta::permissions::Path;
use serde::{Serialize, Serializer};

#[derive(Default, Serialize)]
pub struct Membership {
    pub status: Vec<char>,
}

pub enum Action {
    Add,
    Remove,
}
pub enum Diff {
    Status(char, Action),
}

impl Membership {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Errors
    ///
    /// Will return `Err` if the presented diff is not applicable to the
    /// current network state, or if the result data cannot be serialized.
    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(Path, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::Status(char, action) => {
                let (index, value) = match action {
                    Action::Add => {
                        self.status.push(char);
                        (self.status.len() - 1, char.serialize(ser)?)
                    }
                    Action::Remove => {
                        let index = self
                            .status
                            .iter()
                            .position(|&c| c == char)
                            .ok_or(Error::UnknownMode)?;
                        self.status.remove(index);
                        (index, ser.serialize_none()?)
                    }
                };
                (
                    Path::InternalVertex(
                        "status".to_string(),
                        Box::new(Path::ExternalVertex(index.to_string())),
                    ),
                    value,
                )
            }
        })
    }
}
