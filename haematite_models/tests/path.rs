use haematite_models::meta::permissions::Path;

#[test]
fn test() {
    let path = Path::from("a/b/c/d");

    match path {
        Path::InternalVertex(name, v) if name == "a" => match *v {
            Path::InternalVertex(name, v) if name == "b" => match *v {
                Path::InternalVertex(name, v) if name == "c" => match *v {
                    Path::ExternalVertex(name) if name == "d" => {}
                    v => assert!(false, "wrong d: {:?}", v),
                },
                v => assert!(false, "wrong c: {:?}", v),
            },
            v => assert!(false, "wrong b: {:?}", v),
        },
        v => assert!(false, "wrong a: {:?}", v),
    };
}
