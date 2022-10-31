use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

pub struct SerializeStruct {
    name: &'static str,
    map: HashMap<&'static str, SerializeWrap>,
}

impl SerializeStruct {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            map: HashMap::new(),
        }
    }
}

impl serde::ser::SerializeStruct for SerializeStruct {
    type Ok = WrapType;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer {};
        self.map
            .insert(key, SerializeWrap::new(value.serialize(&mut serializer)?));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(Self::Ok::Struct(self.name, self.map))
    }
}
