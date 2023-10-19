#![allow(dead_code)]
#![feature(exclusive_range_pattern)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(strict_provenance)]

use std::{path::{PathBuf, Path}, fs::{metadata, self}};

use class_loader::ClassLoader;

mod class_loader;
mod jvm;
mod util;

fn main() {
	let jdk_file_paths = list_files(Path::new("./jdk"));
	let jdk: Vec<&str> = jdk_file_paths.iter().map(|pathbuf: &PathBuf| {
		match pathbuf.to_str() {
			Some(v) => v,
			None => panic!("Failed to load JDK"),
		}
	}).collect();

	use std::time::Instant;
    let now = Instant::now();
	{
		ClassLoader::load(vec!["Main.class"], vec![jdk]);
	}
	let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) {
    if metadata(&path).unwrap().is_dir() {
        let paths = fs::read_dir(&path).unwrap();
        for path_result in paths {
            let full_path = path_result.unwrap().path();
            if metadata(&full_path).unwrap().is_dir() {
                _list_files(vec, &full_path);
            } else {
                vec.push(full_path);
            }
        }
    }
}

fn list_files(path: &Path) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    _list_files(&mut vec,&path);
    vec
}