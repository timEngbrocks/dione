use std::collections::HashMap;

use super::{types::object::Object, bootstrap_class_loader::BootstrapClassLoader};

pub struct ObjectManager {
	bootstrap_class_loader: BootstrapClassLoader,
	objects: HashMap<String, Object>,
}

impl ObjectManager {
	pub fn new() -> ObjectManager {
		let bootstrap_class_loader = BootstrapClassLoader::new();
		let objects: HashMap<String, Object> = HashMap::new();

		ObjectManager {
			bootstrap_class_loader,
			objects,
		}
	}

	pub fn initialize(&mut self, jdk_base_path: String) {
		self.bootstrap_class_loader.initialize(jdk_base_path);
		self.load_object("java/lang/Object", true);
		self.load_object("java/lang/Class", true);
		self.load_object("java/lang/System", true);
	}

	pub fn get_object(&self, name: &str) -> &Object {
		match self.objects.get(name) {
			Some(object) => object,
			None => panic!("Object (Class/Interface) `{name}` not found!"),
		}

	}

	fn load_object(&mut self, name: &str, is_jdk_class: bool) {
		let mut objects = self.bootstrap_class_loader.load_object(name, is_jdk_class);
		for object in objects.drain(..) {
			self.objects.insert(object.name.clone(), object);
		}
	}
}