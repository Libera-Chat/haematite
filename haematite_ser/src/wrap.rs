use std::collections::HashMap;

use haematite_models::meta::permissions::Tree;
use serde::ser::{Serialize, Serializer};
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Allow {
    Yes,
    No,
    Untraversable,
}

#[derive(Clone, Debug)]
pub struct SerializeWrap {
    pub inner: WrapType,
    pub allowed: Allow,
}

impl SerializeWrap {
    pub fn new(inner: WrapType) -> Self {
        Self {
            inner,
            allowed: Allow::Yes,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WrapType {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    None,
    Some(Box<WrapType>),
    Unit,
    UnitStruct(&'static str),
    UnitVariant(&'static str, u32, &'static str),
    NewtypeStruct(&'static str, Box<WrapType>),
    NewtypeVariant(&'static str, u32, &'static str, Box<WrapType>),
    Map(HashMap<String, SerializeWrap>),
    Seq(Vec<SerializeWrap>),
    Struct(&'static str, HashMap<&'static str, SerializeWrap>),
    StructVariant(
        &'static str,
        u32,
        &'static str,
        HashMap<&'static str, SerializeWrap>,
    ),
    Tuple(Vec<SerializeWrap>),
    TupleStruct(&'static str, Vec<SerializeWrap>),
    TupleVariant(&'static str, u32, &'static str, Vec<SerializeWrap>),
}

impl WrapType {
    pub fn update_with(&mut self, tree: &Tree) -> Allow {
        match self {
            Self::Map(map) => {
                for (key, value) in map.iter_mut() {
                    if let Some(subtree) = tree.step(key) {
                        value.allowed = value.inner.update_with(subtree);
                    } else {
                        value.allowed = Allow::No;
                    }
                }
                Allow::Yes
            }
            Self::Struct(_, map) | Self::StructVariant(_, _, _, map) => {
                for (key, value) in map.iter_mut() {
                    if let Some(subtree) = tree.step(key) {
                        value.allowed = value.inner.update_with(subtree);
                    } else {
                        value.allowed = Allow::No;
                    }
                }
                Allow::Yes
            }
            Self::Seq(values)
            | Self::Tuple(values)
            | Self::TupleStruct(_, values)
            | Self::TupleVariant(_, _, _, values) => {
                if let Some(subtree) = tree.next() {
                    for value in values.iter_mut() {
                        value.allowed = value.inner.update_with(subtree);
                    }
                    Allow::Yes
                } else {
                    Allow::Untraversable
                }
            }
            _ => Allow::Yes,
        }
    }
}

impl Serialize for WrapType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::I8(value) => serializer.serialize_i8(*value),
            Self::I16(value) => serializer.serialize_i16(*value),
            Self::I32(value) => serializer.serialize_i32(*value),
            Self::I64(value) => serializer.serialize_i64(*value),
            Self::U8(value) => serializer.serialize_u8(*value),
            Self::U16(value) => serializer.serialize_u16(*value),
            Self::U32(value) => serializer.serialize_u32(*value),
            Self::U64(value) => serializer.serialize_u64(*value),
            Self::F32(value) => serializer.serialize_f32(*value),
            Self::F64(value) => serializer.serialize_f64(*value),
            Self::Char(value) => serializer.serialize_char(*value),
            Self::Str(value) => serializer.serialize_str(value),
            Self::Bytes(value) => serializer.serialize_bytes(value),
            Self::None => serializer.serialize_none(),
            Self::Some(value) => serializer.serialize_some(value),
            Self::Unit => serializer.serialize_unit(),
            Self::UnitStruct(name) => serializer.serialize_unit_struct(name),
            Self::UnitVariant(name, variant_index, variant) => {
                serializer.serialize_unit_variant(name, *variant_index, variant)
            }
            Self::NewtypeStruct(name, value) => serializer.serialize_newtype_struct(name, value),
            Self::NewtypeVariant(name, variant_index, variant, value) => {
                serializer.serialize_newtype_variant(name, *variant_index, variant, value)
            }
            Self::Map(map) => {
                let map: HashMap<_, _> = map
                    .iter()
                    .filter(|(_k, v)| v.allowed == Allow::Yes)
                    .collect();
                let mut serializer = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in &map {
                    serializer.serialize_key(key)?;
                    serializer.serialize_value(&value.inner)?;
                }
                serializer.end()
            }
            Self::Seq(values) => {
                let values: Vec<_> = values.iter().filter(|v| v.allowed == Allow::Yes).collect();
                let mut serializer = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    serializer.serialize_element(&value.inner)?;
                }
                serializer.end()
            }
            Self::Struct(name, map) => {
                let map: HashMap<_, _> = map
                    .iter()
                    .filter(|(_k, v)| v.allowed == Allow::Yes)
                    .collect();
                let mut serializer = serializer.serialize_struct(name, map.len())?;
                for (key, value) in &map {
                    serializer.serialize_field(key, &value.inner)?;
                }
                serializer.end()
            }
            Self::StructVariant(name, variant_index, variant, map) => {
                let map: HashMap<_, _> = map
                    .iter()
                    .filter(|(_k, v)| v.allowed == Allow::Yes)
                    .collect();
                let mut serializer = serializer.serialize_struct_variant(
                    name,
                    *variant_index,
                    variant,
                    map.len(),
                )?;
                for (key, value) in &map {
                    serializer.serialize_field(key, &value.inner)?;
                }
                serializer.end()
            }
            Self::Tuple(values) => {
                let values: Vec<_> = values.iter().filter(|v| v.allowed == Allow::Yes).collect();
                let mut serializer = serializer.serialize_tuple(values.len())?;
                for value in values {
                    serializer.serialize_element(&value.inner)?;
                }
                serializer.end()
            }
            Self::TupleStruct(name, values) => {
                let values: Vec<_> = values.iter().filter(|v| v.allowed == Allow::Yes).collect();
                let mut serializer = serializer.serialize_tuple_struct(name, values.len())?;
                for value in values {
                    serializer.serialize_field(&value.inner)?;
                }
                serializer.end()
            }
            Self::TupleVariant(name, variant_index, variant, values) => {
                let values: Vec<_> = values.iter().filter(|v| v.allowed == Allow::Yes).collect();
                let mut serializer = serializer.serialize_tuple_variant(
                    name,
                    *variant_index,
                    variant,
                    values.len(),
                )?;
                for value in values {
                    serializer.serialize_field(&value.inner)?;
                }
                serializer.end()
            }
        }
    }
}
