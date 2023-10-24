use std::collections::HashMap;

use crate::util::heap::Heap;

use super::{types::{object::Object, reference::Reference, ReferenceableTypes}, bootstrap_class_loader::BootstrapClassLoader};

static mut INSTANCE: Option<ObjectManager> = None;

pub struct ObjectManager {
	bootstrap_class_loader: BootstrapClassLoader,
	objects: HashMap<String, Object>,
}

impl ObjectManager {
	pub fn initialize(jdk_base_path: String) {
		let instance = ObjectManager::it();
		instance.initialize_impl(jdk_base_path);
	}

	pub fn load_object(name: &str) {
		let instance = ObjectManager::it();
		instance.load_impl(name, ObjectManager::is_jdk_class(name));
	}

	pub fn get(name: &str) -> &Object {
		let instance = ObjectManager::it();
		instance.get_impl(name)
	}

	pub fn instantiate(name: &str) -> Reference {
		let instance = ObjectManager::it();
		instance.instantiate_impl(name)
	}


	fn it() -> &'static mut ObjectManager {
		unsafe {
			if let Some(_) = INSTANCE {
				INSTANCE.as_mut().unwrap()
			} else {
				INSTANCE = Some(ObjectManager::new());
				INSTANCE.as_mut().unwrap()
			}
		}
	}

	fn new() -> ObjectManager {
		let bootstrap_class_loader = BootstrapClassLoader::new();
		let objects: HashMap<String, Object> = HashMap::new();

		ObjectManager {
			bootstrap_class_loader,
			objects,
		}
	}

	fn initialize_impl(&mut self, jdk_base_path: String) {
		self.bootstrap_class_loader.initialize(jdk_base_path);
		self.load_impl("java/lang/Object", true);
		self.load_impl("java/lang/Class", true);
		self.load_impl("java/lang/System", true);
	}

	fn get_impl(&mut self, name: &str) -> &Object {
		match self.objects.contains_key(name) {
			true => self.objects.get(name).unwrap(),
			false => {
				self.load_impl(name, ObjectManager::is_jdk_class(name));
				self.objects.get(name).expect(format!("ObjectManager::get_object: Could not load object: {}", name).as_str())
			}
		}
	}

	fn load_impl(&mut self, name: &str, is_jdk_class: bool) {
		let mut objects = self.bootstrap_class_loader.load_object(name, is_jdk_class);
		for object in objects.drain(..) {
			self.objects.insert(object.name.clone(), object);
		}
	}

	fn instantiate_impl(&mut self, name: &str) -> Reference {
		let object = self.get_impl(name).clone();
		Heap::allocate(ReferenceableTypes::Class(object))
	}

	fn is_jdk_class(name: &str) -> bool {
		name.starts_with("com/") ||
		name.starts_with("java/") ||
		name.starts_with("javax/") ||
		name.starts_with("jdk/") ||
		name.starts_with("sun/")
	}
}