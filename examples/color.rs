use std::collections::HashMap;

use anstream::println;
use serde::Serialize;
use serde_claplike::to_string;

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MyType {
    unit: (),
    r#struct: MyStruct,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
struct MyStruct {
    string: String,
    option: Option<String>,
    num: usize,
    vec: Vec<&'static str>,
    map: HashMap<&'static str, &'static str>,
}

fn main() {
    let value = MyType {
        unit: (),
        r#struct: MyStruct {
            string: "Hello".into(),
            option: None,
            num: 1337,
            vec: vec!["elliot", "fsociety"],
            map: HashMap::from_iter([("a", "AAAA")]),
        },
    };
    let out = to_string(&value).unwrap();
    println!("{out}");
}
