// #[cfg(debug_assertions)]
extern crate build_const;

use std::path::Path;

fn create_constants() {
    let path = Path::new("/app").join("lingua-rs").join("src").join("consts.rs");
    // panic!("{:?}", path);
    let mut consts = build_const::ConstWriter::from_path(&path)
        .unwrap()
        .finish_dependencies();

    // do some "complex" calculations
    let values: Vec<u8> = vec![1, 2, 3, 36];

    consts.add_value("VALUE", "u8", values.iter().sum::<u8>());
    consts.add_array("ARRAY", "u8", &values);

    let strs = vec!["foo", "bar", "baz"];
    consts.add_array("STRS", "&str", &strs);

    let strs2: Vec<String> = strs
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}: {}", i, s)).collect();
    consts.add_array("STRS2", "&str", &strs2);

    consts.finish();
}

fn main() {
    create_constants();
}
