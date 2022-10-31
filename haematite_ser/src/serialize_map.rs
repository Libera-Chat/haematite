use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::map_key::MapKeySerializer;
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

#[derive(Default)]
pub struct SerializeMap {
    map: HashMap<String, SerializeWrap>,
    key: Option<String>,
}

impl serde::ser::SerializeMap for SerializeMap {
    type Ok = WrapType;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = key.serialize(MapKeySerializer {}).ok();
        Ok(())
    }
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(key) = self.key.take() {
            let mut serializer = Serializer {};
            self.map
                .insert(key, SerializeWrap::new(value.serialize(&mut serializer)?));
        }
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(Self::Ok::Map(self.map))
    }
}
