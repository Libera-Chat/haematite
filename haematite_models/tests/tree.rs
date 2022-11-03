use std::collections::{HashMap, HashSet};

use haematite_models::meta::permissions::{Path, Tree};

#[test]
fn test() {
    let tree = Tree::from(vec![
        Path::from("a"),
        Path::from("a/b"),
        Path::from("a/b/c"),
        Path::from("a/*/d"),
    ]);

    match &tree {
        Tree::InternalVertex(map) => {
            let keys: HashSet<_> = map.keys().collect();
            assert_eq!(keys, HashSet::from([&"a".to_string()]));
            match &map["a"] {
                Tree::InternalVertex(map) => {
                    let keys: HashSet<_> = map.keys().collect();
                    assert_eq!(keys, HashSet::from([&"b".to_string(), &"*".to_string()]));

                    match &map["*"] {
                        Tree::InternalVertex(_map)
                            if matches!(
                                &HashMap::from([(&"d".to_string(), Tree::ExternalVertex)]),
                                _map
                            ) => {}
                        v => assert!(false, "wrong *: {:?}", v),
                    };
                    match &map["b"] {
                        Tree::InternalVertex(_map)
                            if matches!(
                                &HashMap::from([
                                    (&"c".to_string(), Tree::ExternalVertex),
                                    (&"d".to_string(), Tree::ExternalVertex),
                                ]),
                                _map
                            ) => {}
                        v => assert!(false, "wrong c: {:?}", v),
                    };
                }
                v => assert!(false, "wrong b: {:?}", v),
            };

            match map["a"].next() {
                Some(Tree::InternalVertex(_map))
                    if matches!(
                        &HashMap::from([(&"d".to_string(), Tree::ExternalVertex)]),
                        _map
                    ) => {}
                v => assert!(false, "wrong next: {:?}", v),
            };
            match map["a"].step("b") {
                Some(Tree::InternalVertex(_map))
                    if matches!(
                        &HashMap::from([
                            (&"c".to_string(), Tree::ExternalVertex),
                            (&"d".to_string(), Tree::ExternalVertex),
                        ]),
                        _map
                    ) => {}
                v => assert!(false, "wrong step: {:?}", v),
            };
        }
        v => assert!(false, "wrong a: {:?}", v),
    };

    match tree.walk(&Path::from("a/b")) {
        Some(Tree::InternalVertex(_map))
            if matches!(
                &HashMap::from([
                    (&"c".to_string(), Tree::ExternalVertex),
                    (&"d".to_string(), Tree::ExternalVertex),
                ]),
                _map
            ) => {}
        v => assert!(false, "wrong walk: {:?}", v),
    };
}
