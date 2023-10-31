use std::collections::HashMap;

use crate::util::heap::Heap;

use super::{types::{object::Object, reference::Reference, ReferenceableTypes}, bootstrap_class_loader::BootstrapClassLoader, interpreter::Interpreter};

static mut INSTANCE: Option<ObjectManager> = None;

pub struct ObjectManager {
	bootstrap_class_loader: BootstrapClassLoader,
	objects: HashMap<String, Object>,
	initialized: HashMap<String, bool>,
	being_initialized: HashMap<String, bool>,
	jdk_base_path: String,
}

impl ObjectManager {
	pub fn initialize(jdk_base_path: String) {
		unsafe {
			INSTANCE = Some(ObjectManager::new(jdk_base_path));
		}
	}

	pub fn load_object(name: &str) {
		let instance = ObjectManager::it();
		instance.load_impl(name);
	}

	pub fn initialize_object(name: &str) {
		let instance = ObjectManager::it();
		instance.initialize_impl(name);
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
				panic!("ObjectManager has not been initialized")
			}
		}
	}

	fn new(jdk_base_path: String) -> ObjectManager {
		let mut bootstrap_class_loader = BootstrapClassLoader::new();
		bootstrap_class_loader.initialize(jdk_base_path.clone());
		let objects: HashMap<String, Object> = HashMap::new();
		let initialized: HashMap<String, bool> = HashMap::new();
		let being_initialized: HashMap<String, bool> = HashMap::new();

		ObjectManager {
			bootstrap_class_loader,
			objects,
			initialized,
			being_initialized,
			jdk_base_path,
		}
	}

	fn get_impl(&mut self, name: &str) -> &Object {
		match self.objects.contains_key(name) {
			true => {
				match self.initialized.get(name) {
					Some(true) => {
						self.objects.get(name).unwrap()
					},
					_ => panic!("Tried to get uninitialized object: {}", name),
				}
			},
			false => panic!("Object not found: {}", name),
		}
	}

	fn load_impl(&mut self, name: &str) {
		let mut objects = self.bootstrap_class_loader.load_object(name);
		for object in objects.drain(..) {
			self.initialized.insert(object.name.clone(), false);
			self.objects.insert(object.name.clone(), object);
		}
	}

	fn initialize_impl(&mut self, name: &str) {
		match self.objects.contains_key(name) {
			true => {},
			false => self.load_impl(name),
		}
		match self.initialized.get(name) {
			Some(initialized) => {
				if !initialized {
					if let Some(true) = self.being_initialized.get(name) {
						return;
					}
					self.being_initialized.insert(name.to_string(), true);
					let object = self.objects.get(name).unwrap();
					if let Some(_) = object.get_method("<clinit>", "()V") {
						let mut interpreter = Interpreter::new();
						interpreter.run(name, "<clinit>", "()V");	
					}
					self.initialized.insert(name.to_string(), true);
					self.being_initialized.remove_entry(name);
				}
			},
			_ => panic!("Error initializing object: {}", name),
		};
	}

	fn instantiate_impl(&mut self, name: &str) -> Reference {
		let object = self.get_impl(name).clone();
		Heap::allocate(ReferenceableTypes::Class(object))
	}
}