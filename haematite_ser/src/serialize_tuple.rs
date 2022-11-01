use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

pub struct SerializeTuple {
    seq: Vec<SerializeWrap>,
}

impl SerializeTuple {
    pub fn new(len: usize) -> Self {
        Self {
            seq: Vec::with_capacity(len),
        }
    }
}

impl serde::ser::SerializeTuple for SerializeTuple {
    type Ok = WrapType;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer {};
        self.seq
            .push(SerializeWrap::new(value.serialize(&mut serializer)?));
        Ok(())
    }
    fn end(self) -> Result<Self::Ok> {
        Ok(Self::Ok::Tuple(self.seq))
    }
}
