use super::error::Error;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub users: Vec<String>,
}

pub enum Action {
    Add,
    Remove,
}

pub enum Diff {
    User(String, Action),
}

impl Server {
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            ..Self::default()
        }
    }

    pub fn update<S>(&mut self, diff: Diff, ser: S) -> Result<(String, S::Ok), Error>
    where
        S: Serializer,
    {
        Ok(match diff {
            Diff::User(uid, action) => {
                let (index, value) = match action {
                    Action::Add => {
                        let value = uid.serialize(ser)?;
                        self.users.push(uid);
                        (self.users.len() - 1, value)
                    }
                    Action::Remove => {
                        let index = self
                            .users
                            .iter()
                            .position(|u| u == &uid)
                            .ok_or(Error::UnknownUser)?;
                        self.users.remove(index);
                        (index, ser.serialize_none()?)
                    }
                };
                (format!("users/{}", index), value)
            }
        })
    }
}
