use std::fs;

use self::class_file::ClassFile;

pub mod parser;

mod attribute_info;
mod constant_pool_info;
mod field_info;
mod method_info;
mod class_file;

pub struct ClassLoader {
	pub class_files: Vec<ClassFile>
}

impl ClassLoader {
	pub fn load(files: Vec<&str>, libraries: Vec<Vec<&str>>) -> ClassLoader {
		let mut class_files: Vec<ClassFile> = files.iter().map(|path| ClassFile::parse(ClassLoader::load_file(path))).collect();
		libraries.iter().for_each(|paths| {
			paths.iter().for_each(|path| {
				class_files.push(ClassFile::parse(ClassLoader::load_file(path)));
			});
		});
		println!("Loaded {} classes", class_files.len());
		ClassLoader { class_files }
	}
	
	fn load_file(path: &str) -> Vec<u8> {
		match fs::read(path) {
			Ok(data) => data,
			Err(error) => panic!("{error}")
		}
	}
}