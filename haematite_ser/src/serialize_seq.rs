use crate::error::{Error, Result};
use crate::wrap::{SerializeWrap, WrapType};
use crate::Serializer;

use serde::ser::Serialize;

#[derive(Default)]
pub struct SerializeSeq {
    seq: Vec<SerializeWrap>,
}

impl serde::ser::SerializeSeq for SerializeSeq {
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
        Ok(Self::Ok::Seq(self.seq))
    }
}
