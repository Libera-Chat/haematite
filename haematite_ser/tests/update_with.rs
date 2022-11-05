use std::collections::{HashMap, HashSet};

use serde::Serialize;

use haematite_models::meta::permissions::{Path, Tree};
use haematite_ser::wrap::Allow;
use haematite_ser::{Serializer, WrapType};

#[derive(Serialize)]
struct TestStruct {
    j: u8,
    i: u8,
}

#[test]
fn _struct() {
    let obj = TestStruct { i: 5, j: 6 };

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("j")]));
    assert_eq!(out, Allow::Yes);

    match wrap {
        WrapType::Struct(_, map) => {
            let keys: HashSet<_> = map.keys().collect();
            assert_eq!(HashSet::from([&"i", &"j"]), keys);

            assert_eq!(map["i"].allowed, Allow::No);
            assert_eq!(map["j"].allowed, Allow::Yes);
        }
        v => assert!(false, "wrong hashmap: {:?}", v),
    }
}

#[test]
fn hashmap() {
    let obj = HashMap::from([("j".to_string(), 5), ("i".to_string(), 6)]);

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("j")]));
    assert_eq!(out, Allow::Yes);

    match wrap {
        WrapType::Map(map) => {
            let keys: HashSet<_> = map.keys().collect();
            assert_eq!(HashSet::from([&"i".to_string(), &"j".to_string()]), keys);

            assert_eq!(map["i"].allowed, Allow::No);
            assert_eq!(map["j"].allowed, Allow::Yes);
        }
        v => assert!(false, "wrong hashmap: {:?}", v),
    }
}

#[test]
fn seq_traversable() {
    let obj = Vec::from([5]);

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("*")]));
    assert_eq!(out, Allow::Yes);

    match wrap {
        WrapType::Seq(seq) => {
            assert_eq!(seq.len(), 1);
            assert_eq!(seq[0].allowed, Allow::Yes);
        }
        v => assert!(false, "wrong seq: {:?}", v),
    }
}

#[test]
fn seq_untraversable() {
    let obj = Vec::from([5]);

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("1")]));
    assert_eq!(out, Allow::Untraversable);
}

#[test]
fn tuple_traversable() {
    let obj = (5,);

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("*")]));
    assert_eq!(out, Allow::Yes);

    match wrap {
        WrapType::Tuple(seq) => {
            assert_eq!(seq.len(), 1);
            assert_eq!(seq[0].allowed, Allow::Yes);
        }
        v => assert!(false, "wrong tuple: {:?}", v),
    }
}

#[test]
fn tuple_untraversable() {
    let obj = (5,);

    let mut serializer = Serializer {};
    let mut wrap = obj.serialize(&mut serializer).unwrap();
    let out = wrap.update_with(&Tree::from(vec![Path::from("1")]));
    assert_eq!(out, Allow::Untraversable);
}
