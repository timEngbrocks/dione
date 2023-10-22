use std::collections::HashSet;

use crate::{util::queue::Queue, class_loader::{class_file::ClassFile, self}};

use super::types::object::Object;

mod object_initializer;

pub struct BootstrapClassLoader {
	to_load: Queue<String>,
	loaded: HashSet<String>,
	jdk_base_path: String,
}

impl BootstrapClassLoader {
	pub fn new() -> Self {
		let to_load = Queue::new(true);
		let loaded = HashSet::new();

		BootstrapClassLoader {
			to_load,
			loaded,
			jdk_base_path: String::new(),
		}
	}

	pub fn initialize(&mut self, jdk_base_path: String) {
		self.jdk_base_path = jdk_base_path;
	}

	pub fn load_object(&mut self, name: &str, is_jdk_class: bool) -> Vec<Object> {
		if self.loaded.contains(name) {
			return Vec::new();
		}
		let mut objects = Vec::new();
		self.to_load.enqueue(name.to_string());
		while let Some(name) = self.to_load.dequeue() {
			let path = self.resolve_path(&name, is_jdk_class);
			let class_file = self.load_class_file(path);
			let object = match class_file.is_class() {
				true => self.initialize_class(class_file),
				false => self.initialize_interface(class_file),
			};
			objects.push(object);
		}
		objects
	}

	fn resolve_path(&self, name: &str, is_jdk_class: bool) -> String {
		if is_jdk_class {
			format!("{}/{}.class", self.jdk_base_path, name)
		} else {
			format!("{}/{}.class", self.jdk_base_path, name)
		}
	}

	fn load_class_file(&self, path: String) -> ClassFile {
		class_loader::load(path)
	}

	fn initialize_class(&self, class_file: ClassFile) -> Object {
		object_initializer::initialize_class(class_file)
	}

	fn initialize_interface(&self, class_file: ClassFile) -> Object {
		object_initializer::initialize_interface(class_file)
	}
}