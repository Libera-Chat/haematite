pub mod error;
mod map_key;

pub mod serialize_map;
pub mod serialize_seq;
pub mod serialize_struct;
pub mod serialize_struct_variant;
pub mod serialize_tuple;
pub mod serialize_tuple_struct;
pub mod serialize_tuple_variant;
pub mod wrap;

use crate::error::{Error, Result};
use crate::serialize_map::SerializeMap;
use crate::serialize_seq::SerializeSeq;
use crate::serialize_struct::SerializeStruct;
use crate::serialize_struct_variant::SerializeStructVariant;
use crate::serialize_tuple::SerializeTuple;
use crate::serialize_tuple_struct::SerializeTupleStruct;
use crate::serialize_tuple_variant::SerializeTupleVariant;
pub use crate::wrap::WrapType;

use serde::Serialize;

pub struct Serializer {}

impl serde::ser::Serializer for &mut Serializer {
    type Ok = WrapType;
    type Error = Error;

    type SerializeMap = SerializeMap;
    type SerializeSeq = SerializeSeq;
    type SerializeStruct = SerializeStruct;
    type SerializeStructVariant = SerializeStructVariant;
    type SerializeTuple = SerializeTuple;
    type SerializeTupleStruct = SerializeTupleStruct;
    type SerializeTupleVariant = SerializeTupleVariant;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        Ok(WrapType::Bool(value))
    }
    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        Ok(WrapType::I8(value))
    }
    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        Ok(WrapType::I16(value))
    }
    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        Ok(WrapType::I32(value))
    }
    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        Ok(WrapType::I64(value))
    }
    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        Ok(WrapType::U8(value))
    }
    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        Ok(WrapType::U16(value))
    }
    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        Ok(WrapType::U32(value))
    }
    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        Ok(WrapType::U64(value))
    }
    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        Ok(WrapType::F32(value))
    }
    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        Ok(WrapType::F64(value))
    }
    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        Ok(WrapType::Char(value))
    }
    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(WrapType::Str(value.to_owned()))
    }
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(WrapType::Bytes(value.to_vec()))
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(WrapType::None)
    }
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Ok(WrapType::Some(Box::new(value.serialize(self)?)))
    }
    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(WrapType::Unit)
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(WrapType::UnitStruct(name))
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(WrapType::UnitVariant(name, variant_index, variant))
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Ok(WrapType::NewtypeStruct(
            name,
            Box::new(value.serialize(self)?),
        ))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Ok(WrapType::NewtypeVariant(
            name,
            variant_index,
            variant,
            Box::new(value.serialize(self)?),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeSeq::default())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeTuple::default())
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SerializeTupleStruct::new(name))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SerializeTupleVariant::new(name, variant_index, variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::default())
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(SerializeStruct::new(name))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeStructVariant::new(name, variant_index, variant))
    }
}
