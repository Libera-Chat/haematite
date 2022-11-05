use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

pub struct SerializeTupleStruct {
    name: &'static str,
    seq: Vec<SerializeWrap>,
}

impl SerializeTupleStruct {
    pub fn new(name: &'static str, len: usize) -> Self {
        Self {
            name,
            seq: Vec::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeTupleStruct for SerializeTupleStruct {
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
        Ok(Self::Ok::TupleStruct(self.name, self.seq))
    }
}
