use crate::util::heap::Heap;

use self::{object_manager::ObjectManager, interpreter::Interpreter};

pub mod instructions;
pub mod frame;
pub mod types;
pub mod runtime_constant_pool;
pub mod bootstrap_class_loader;
pub mod object_manager;
pub mod interpreter;
pub mod execution_context;
pub mod descriptor;

static mut INSTANCE: Option<JVM> = None;

pub struct JVM {
	pub interpreter: Interpreter,
}

impl JVM {
	pub fn start(jdk_base_path: String) {
		ObjectManager::initialize(jdk_base_path);
		Heap::initialize();

		unsafe {
			INSTANCE = Some(JVM {
				interpreter: Interpreter::new(),
			});
		}
	}

	pub fn run(_: Vec<String>) {
		unimplemented!()
	}

	fn it() -> &'static mut JVM {
		// FIXME: thread-safe singleton
		unsafe {
			if let Some(jvm) = INSTANCE.as_mut() {
				jvm
			} else {
				panic!("JVM has not been started")
			}
		}
	}
}