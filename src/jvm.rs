use self::object_manager::ObjectManager;

pub mod instructions;
pub mod frame;
pub mod types;
pub mod runtime_constant_pool;
pub mod bootstrap_class_loader;
pub mod object_manager;

static mut JVM_INSTANCE: Option<JVM> = None;

pub struct JVM {
	pub object_manager: ObjectManager,
}

impl JVM {
	pub fn it() -> &'static mut JVM {
		// FIXME: thread-safe singleton
		unsafe {
			if let Some(jvm) = JVM_INSTANCE.as_mut() {
				jvm
			} else {
				JVM_INSTANCE = Some(JVM::new());
				JVM_INSTANCE.as_mut().unwrap()
			}
		}
	}

	pub fn new() -> JVM {
		let object_manager = ObjectManager::new();

		JVM {
			object_manager,
		}
	}

	pub fn initialize(&mut self, jdk_base_path: String) {
		self.object_manager.initialize(jdk_base_path);
	}

	pub fn run(&self, _names: Vec<String>) {
		unimplemented!("JVM::run")
	}
}