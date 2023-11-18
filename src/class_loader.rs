use std::fs;

use self::class_file::ClassFile;

pub mod attribute_info;
pub mod class_file;
pub mod constant_pool_info;
pub mod field_info;
pub mod method_info;
pub mod parser;

pub fn load(path: String) -> ClassFile {
    ClassFile::parse(load_file(path))
}

fn load_file(path: String) -> Vec<u8> {
    match fs::read(path.clone()) {
        Ok(data) => data,
        Err(error) => panic!("Could not read file at {}: {}", path, error),
    }
}
