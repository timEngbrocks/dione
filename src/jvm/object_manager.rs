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

	pub fn get_object(&mut self, name: &str) -> &Object {
		match self.objects.contains_key(name) {
			true => self.objects.get(name).unwrap(),
			false => {
				self.load_object(name, ObjectManager::is_jdk_class(name));
				self.objects.get(name).expect(format!("ObjectManager::get_object: Could not load object: {}", name).as_str())
			}
		}
	}

	fn load_object(&mut self, name: &str, is_jdk_class: bool) {
		let mut objects = self.bootstrap_class_loader.load_object(name, is_jdk_class);
		for object in objects.drain(..) {
			self.objects.insert(object.name.clone(), object);
		}
	}

	fn is_jdk_class(name: &str) -> bool {
		name.starts_with("com/") ||
		name.starts_with("java/") ||
		name.starts_with("javax/") ||
		name.starts_with("jdk/") ||
		name.starts_with("sun/")
	}
}