use std::{path::{PathBuf, Path}, fs::{metadata, self}};

pub fn list_files(path: &Path) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    _list_files(&mut vec,path);
    vec
}

fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) {
    if metadata(path).unwrap().is_dir() {
        let paths = fs::read_dir(path).unwrap();
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