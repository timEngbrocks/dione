use std::fs;

use class_file::{class_file_parser, ClassFile};

pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

pub mod class_file;
pub mod cp_info;
pub mod field_info;
pub mod method_info;
pub mod attribute_info;

pub fn parse(path: String) -> ClassFile {
    let class_file_raw = match fs::read(&path) {
        Ok(v) => v,
        Err(e) => panic!("Failed to read class file at '{path}'. Got: '{e}'"),
    };

    let (rest, class_file) = match class_file_parser::<()>(&class_file_raw[..]) {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse class file. Got: '{e}'"),
    };

    if !rest.is_empty() {
        panic!("Failed to parse complete class file. Have {} bytes remaining!", rest.len());
    }

    class_file
}
