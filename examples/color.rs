use anstream::println;
use serde::Serialize;
use serde_claplike::to_string;

#[derive(Default, Serialize)]
struct MyType {
    my_unit: (),
    my_bool: bool,
    my_int: isize,
    my_uint: usize,
    my_string: String,
    my_option: Option<String>,
    my_struct: MyStruct,
}

#[derive(Default, Serialize)]
struct MyStruct {
    foo: String,
}

fn main() {
    let out = to_string(&MyType::default()).unwrap();
    println!("{out}");
}
