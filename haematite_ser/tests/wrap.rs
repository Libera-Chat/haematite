use std::collections::HashMap;
use std::ops::RangeBounds as _;

use serde::Serialize;

use haematite_ser::wrap::{Allow, WrapType};
use haematite_ser::Serializer;

#[derive(Serialize)]
struct TestNewtypeStruct(u8);

#[test]
fn simple() {
    let mut serializer = Serializer {};

    assert!(matches!(
        false.serialize(&mut serializer),
        Ok(WrapType::Bool(false))
    ));

    assert!(matches!(
        5_u8.serialize(&mut serializer),
        Ok(WrapType::U8(5))
    ));
    assert!(matches!(
        5_u16.serialize(&mut serializer),
        Ok(WrapType::U16(5))
    ));
    assert!(matches!(
        5_u32.serialize(&mut serializer),
        Ok(WrapType::U32(5))
    ));
    assert!(matches!(
        5_u64.serialize(&mut serializer),
        Ok(WrapType::U64(5))
    ));

    assert!(matches!(
        5_i8.serialize(&mut serializer),
        Ok(WrapType::I8(5))
    ));
    assert!(matches!(
        5_i16.serialize(&mut serializer),
        Ok(WrapType::I16(5))
    ));
    assert!(matches!(
        5_i32.serialize(&mut serializer),
        Ok(WrapType::I32(5))
    ));
    assert!(matches!(
        5_i64.serialize(&mut serializer),
        Ok(WrapType::I64(5))
    ));

    match 5_f32.serialize(&mut serializer) {
        Ok(WrapType::F32(n)) => assert_eq!(5_f32, n, "f32 wrong inner: {}", n),
        v => assert!(false, "f32 wrong outer: {:?}", v),
    };
    match 5_f64.serialize(&mut serializer) {
        Ok(WrapType::F64(n)) => assert_eq!(5_f64, n, "f64 wrong inner: {}", n),
        v => assert!(false, "f64 wrong outer: {:?}", v),
    };

    assert!(matches!(
        'j'.serialize(&mut serializer),
        Ok(WrapType::Char('j'))
    ));
    match "j".serialize(&mut serializer) {
        Ok(WrapType::Str(s)) => assert_eq!("j", s, "str wrong inner: {}", s),
        v => assert!(false, "str wrong outer: {:?}", v),
    }
    match std::ffi::CStr::from_bytes_with_nul(b"j\0")
        .unwrap()
        .serialize(&mut serializer)
    {
        Ok(WrapType::Bytes(b)) => assert_eq!(Vec::from([b'j']), b, "bytes wrong inner: {:?}", b),
        v => assert!(false, "bytes wrong outer: {:?}", v),
    }

    assert!(matches!(
        Option::<u8>::None.serialize(&mut serializer),
        Ok(WrapType::None)
    ));
    match Some(5_u8).serialize(&mut serializer) {
        Ok(WrapType::Some(v)) => match *v {
            WrapType::U8(n) => assert_eq!(5, n, "some wrong inner: {}", n),
            v => assert!(false, "some wrong middle: {:?}", v),
        },
        v => assert!(false, "some wrong outer: {:?}", v),
    };
    assert!(matches!(().serialize(&mut serializer), Ok(WrapType::Unit)));
    let phantom: std::marker::PhantomData<u8> = std::marker::PhantomData;
    assert!(matches!(
        phantom.serialize(&mut serializer),
        Ok(WrapType::UnitStruct("PhantomData"))
    ));

    assert!(matches!(
        (..5).start_bound().serialize(&mut serializer),
        Ok(WrapType::UnitVariant("Bound", 0, "Unbounded"))
    ));

    match TestNewtypeStruct(5).serialize(&mut serializer) {
        Ok(WrapType::NewtypeStruct("TestNewtypeStruct", v)) => match *v {
            WrapType::U8(v) => assert_eq!(5, v, "newtype_struct wrong inner: {}", v),
            v => assert!(false, "newtype_struct wrong middle: {:?}", v),
        },
        v => assert!(false, "newtype_struct wrong outer: {:?}", v),
    };

    match Result::<u8, u8>::Ok(5).serialize(&mut serializer) {
        Ok(WrapType::NewtypeVariant("Result", 0, "Ok", v)) => match *v {
            WrapType::U8(v) => assert_eq!(5, v, "newtype_variant wrong inner: {}", v),
            v => assert!(false, "newtype_variant wrong middle: {:?}", v),
        },
        v => assert!(false, "newtype_variant wrong outer: {:?}", v),
    };
}

#[test]
fn map() {
    let mut serializer = Serializer {};

    match HashMap::from([("j", 5_u8)]).serialize(&mut serializer) {
        Ok(WrapType::Map(map)) => {
            let keys: Vec<&String> = map.keys().collect();
            assert_eq!(keys, &["j"]);
            let value = &map["j"];
            assert_eq!(value.allowed, Allow::Yes);
            assert!(matches!(value.inner, WrapType::U8(5)))
        }
        v => assert!(false, "map wrong outer: {:?}", v),
    }
}

#[test]
fn seq() {
    let mut serializer = Serializer {};

    match Vec::from([5_u8]).serialize(&mut serializer) {
        Ok(WrapType::Seq(seq)) => {
            assert_eq!(seq.len(), 1);
            let item = &seq[0];
            assert_eq!(item.allowed, Allow::Yes);
            assert!(matches!(item.inner, WrapType::U8(5)));
        }
        v => assert!(false, "seq wrong outer: {:?}", v),
    }
}

#[derive(Serialize)]
struct TestStruct {
    j: u8,
}

#[test]
fn _struct() {
    let mut serializer = Serializer {};

    match (TestStruct { j: 5 }).serialize(&mut serializer) {
        Ok(WrapType::Struct("TestStruct", map)) => {
            let keys: Vec<&&str> = map.keys().collect();
            assert_eq!(keys, &[&"j"]);
            let value = &map["j"];
            assert_eq!(value.allowed, Allow::Yes);
            assert!(matches!(value.inner, WrapType::U8(5)))
        }
        v => assert!(false, "struct wrong outer: {:?}", v),
    }
}

#[derive(Serialize)]
enum TestStructVariant {
    J { j: u8 },
}

#[test]
fn struct_variant() {
    let mut serializer = Serializer {};

    match (TestStructVariant::J { j: 5 }).serialize(&mut serializer) {
        Ok(WrapType::StructVariant("TestStructVariant", 0, "J", map)) => {
            let keys: Vec<&&str> = map.keys().collect();
            assert_eq!(keys, &[&"j"]);
            let value = &map["j"];
            assert_eq!(value.allowed, Allow::Yes);
            assert!(matches!(value.inner, WrapType::U8(5)))
        }
        v => assert!(false, "struct_variant wrong outer: {:?}", v),
    }
}

#[test]
fn tuple() {
    let mut serializer = Serializer {};

    match ("j", 5_u8).serialize(&mut serializer) {
        Ok(WrapType::Tuple(seq)) => {
            assert_eq!(seq.len(), 2);
            assert_eq!(seq[0].allowed, Allow::Yes);
            match &seq[0].inner {
                WrapType::Str(s) => assert_eq!(s, &"j".to_string()),
                v => assert!(false, "tuple wrong inner 0: {:?}", v),
            }
            assert_eq!(seq[1].allowed, Allow::Yes);
            match &seq[1].inner {
                WrapType::U8(5) => {}
                v => assert!(false, "tuple wrong inner 1: {:?}", v),
            }
        }
        v => assert!(false, "tuple wrong outer: {:?}", v),
    }
}

#[derive(Serialize)]
struct TestTupleStruct(String, u8);

#[test]
fn tuple_struct() {
    let mut serializer = Serializer {};

    match TestTupleStruct("j".to_string(), 5_u8).serialize(&mut serializer) {
        Ok(WrapType::TupleStruct("TestTupleStruct", seq)) => {
            assert_eq!(seq.len(), 2);
            assert_eq!(seq[0].allowed, Allow::Yes);
            match &seq[0].inner {
                WrapType::Str(s) => assert_eq!(s, &"j".to_string()),
                v => assert!(false, "tuple_struct wrong inner 0: {:?}", v),
            }
            assert_eq!(seq[1].allowed, Allow::Yes);
            match &seq[1].inner {
                WrapType::U8(5) => {}
                v => assert!(false, "tuple_struct wrong inner 1: {:?}", v),
            }
        }
        v => assert!(false, "tuple_struct wrong outer: {:?}", v),
    }
}

#[derive(Serialize)]
enum TestTupleVariant {
    J(String, u8),
}

#[test]
fn tuple_variant() {
    let mut serializer = Serializer {};

    match TestTupleVariant::J("j".to_string(), 5_u8).serialize(&mut serializer) {
        Ok(WrapType::TupleVariant("TestTupleVariant", 0, "J", seq)) => {
            assert_eq!(seq.len(), 2);
            assert_eq!(seq[0].allowed, Allow::Yes);
            match &seq[0].inner {
                WrapType::Str(s) => assert_eq!(s, &"j".to_string()),
                v => assert!(false, "tuple_struct wrong inner 0: {:?}", v),
            }
            assert_eq!(seq[1].allowed, Allow::Yes);
            match &seq[1].inner {
                WrapType::U8(5) => {}
                v => assert!(false, "tuple_struct wrong inner 1: {:?}", v),
            }
        }
        v => assert!(false, "tuple_struct wrong outer: {:?}", v),
    }
}
