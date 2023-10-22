pub struct PackagePath {
	path: Vec<String>,
}

impl PackagePath {
	pub fn new(path: Vec<String>) -> Self {
		PackagePath {
			path: path,
		}
	}

	pub fn get_path(&self) -> String {
		self.path.join("/")
	}
}

pub struct FullyQualifiedClassName {
	package_path: PackagePath,
	class_name: String,
}

pub struct ClassFilePaths {
	base_dirs: Vec<&str>,
}

impl ClassFilePath {
	pub fn new(base_dirs: Vec<&str>) -> Self {
		ClassFilePath {}
	}

	pub fn get_class_file_path(&self, class_name: &FullyQualifiedClassName) -> String {

	}


}
