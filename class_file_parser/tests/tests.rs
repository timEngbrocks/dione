use class_file_parser::class_file::class_file_parser;
use std::fs;

const CLASS_FILE_PATH: &str = "./Main.class";

#[test]
fn parse_class_file() {
    let class_file_raw = match fs::read(CLASS_FILE_PATH) {
        Ok(v) => v,
        Err(e) => panic!("Failed to read class file. Got: '{e}'"),
    };

    let (rest, _class_file) = match class_file_parser::<()>(&class_file_raw[..]) {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse class file. Got: '{e}'"),
    };
    if !rest.is_empty() {
        panic!(
            "Failed to parse complete class file. Have {} bytes remaining!",
            rest.len()
        );
    }
}
