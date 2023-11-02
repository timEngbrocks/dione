use std::collections::HashMap;

use crate::util::heap::Heap;

use super::{types::{object::Object, reference::Reference}, bootstrap_class_loader::BootstrapClassLoader, interpreter::Interpreter};

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

	pub fn get(name: &str) -> &mut Object {
		let instance = ObjectManager::it();
		instance.get_impl(name)
	}

	pub fn instantiate(name: &str) -> Reference {
		let instance = ObjectManager::it();
		instance.instantiate_impl(name)
	}

	pub fn is_class(name: &str) -> bool {
		if name.starts_with("[") {
			return false;
		}
		match ObjectManager::get(name).fields.capacity() {
			0 => false,
			_ => true,
		}
	}

	pub fn is_array_class(name: &str) -> bool {
		if name.starts_with("[") {
			return true;
		}
		false
	}

	pub fn is_interface(name: &str) -> bool {
		if name.starts_with("[") {
			return false;
		}
		match ObjectManager::get(name).fields.capacity() {
			0 => true,
			_ => false,
		}
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

	fn get_impl(&mut self, name: &str) -> &mut Object {
		match self.objects.contains_key(name) {
			true => {
				match self.initialized.contains_key(name) {
					true => {
						self.objects.get_mut(name).unwrap()
					},
					false => {
						match self.being_initialized.contains_key(name) {
							true => {
								self.objects.get_mut(name).unwrap()
							},
							false => panic!("Object not initialized or being initialized: {}", name)
						}
					}
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
		match object.fields.capacity() {
			0 => {
				Heap::allocate_interface(object)
			},
			_ => {
				Heap::allocate_class(object)
			},
		}
	}
}