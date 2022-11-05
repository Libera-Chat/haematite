use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

pub struct SerializeTupleVariant {
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    seq: Vec<SerializeWrap>,
}

impl SerializeTupleVariant {
    pub fn new(name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Self {
        Self {
            name,
            variant_index,
            variant,
            seq: Vec::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = WrapType;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer {};
        self.seq
            .push(SerializeWrap::new(value.serialize(&mut serializer)?));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(Self::Ok::TupleVariant(
            self.name,
            self.variant_index,
            self.variant,
            self.seq,
        ))
    }
}
