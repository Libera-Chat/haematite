use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

pub struct SerializeStructVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    map: HashMap<&'static str, SerializeWrap>,
}

impl SerializeStructVariant {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            map: HashMap::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeStructVariant for SerializeStructVariant {
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
        Ok(Self::Ok::StructVariant(
            self.name,
            self.variant_index,
            self.variant,
            self.map,
        ))
    }
}
