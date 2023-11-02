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
pub mod exception_handler;
pub mod native;

static mut INSTANCE: Option<JVM> = None;

pub struct JVM {
	pub interpreter: Interpreter,
}

impl JVM {
	pub fn start(jdk_base_path: String) {
		JVM::setup_logging();

		unsafe {
			INSTANCE = Some(JVM {
				interpreter: Interpreter::new(),
			});
		}

		JVM::it().initialize(jdk_base_path);
	}

	pub fn run(classes: Vec<String>) {
		for class in classes {
			ObjectManager::load_object(&class);
		}
		let jvm = JVM::it();
		jvm.interpreter.start();
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

	// FIXME: VMUniverse status
	fn initialize(&mut self, jdk_base_path: String) {
		// TODO: setup Interpreter log

		self.initialize_globals();

		// FIXME: ObjectMonitor::initialize_object
		// FIXME: ObjectSynchronizer::initialize_object
		
		self.initialize_modules(jdk_base_path);

		// FIXME: Create VMThread
			// Create System Thread group
			// Create Primordial Thread
			// Add new thread and start it
		
		self.initialize_java_lang_classes();
		// FIXME: MutexLocker::post_initialize
		// FIXME: ServiceThread::initialize_object
		// FIXME: MonitorDeflationThread::initialize_object

		self.initialize_jsr292_classes();

		// FIXME: System.initPhase2()
		// FIXME: System.initPhase3()

		// FIXME: SystemDictonary::compute_java_loaders

		self.call_post_vm_init_hook();

		// FIXME: Start WatcherThread
	}

	fn initialize_globals(&self) {
		// TODO: Mutex init
	}

	fn initialize_modules(&self, jdk_base_path: String) {
		// TODO: Setup all global managers
		ObjectManager::initialize(jdk_base_path);
		Heap::initialize();
		
		// FIXME: Initialize finalizer
		// FIXME: Initialize code cache
		// FIXME: Initialize GC barrier

		self.genesis();
	}

	fn initialize_java_lang_classes(&self) {
		ObjectManager::initialize_object("java/lang/String");
		ObjectManager::initialize_object("java/lang/System");
		ObjectManager::initialize_object("java/lang/Class");

		// FIXME: create initial thread group -> VMUniverse
		ObjectManager::initialize_object("java/lang/ThreadGroup");
		// FIXME: create initial thread
		ObjectManager::initialize_object("java/lang/Thread");

		ObjectManager::initialize_object("java/lang/Module");

		// FIXME: set unsafe constants
		ObjectManager::initialize_object("jdk/internal/misc/UnsafeConstants");
		ObjectManager::initialize_object("java/lang/reflect/Method");
		ObjectManager::initialize_object("java/lang/ref/Finalizer");

		// FIXME: System.initPhase1()

		ObjectManager::initialize_object("java/lang/OutOfMemoryError");
		ObjectManager::initialize_object("java/lang/NullPointerException");
		ObjectManager::initialize_object("java/lang/ClassCastException");
		ObjectManager::initialize_object("java/lang/ArrayStoreException");
		ObjectManager::initialize_object("java/lang/ArithmeticException");
		ObjectManager::initialize_object("java/lang/StackOverflowError");
		ObjectManager::initialize_object("java/lang/IllegalMonitorStateException");
		ObjectManager::initialize_object("java/lang/IllegalArgumentException");
	}

	fn initialize_jsr292_classes(&self) {
		ObjectManager::initialize_object("java/lang/invoke/MethodHandle");
		ObjectManager::initialize_object("java/lang/invoke/ResolvedMethodName");
		ObjectManager::initialize_object("java/lang/invoke/MemberName");
		ObjectManager::initialize_object("java/lang/invoke/MethodHandleNatives");
	}

	fn call_post_vm_init_hook(&self) {
		ObjectManager::initialize_object("jdk/internal/vm/PostVMInitHook");
		
		// FIXME: call static: run -> ()V
	}

	fn genesis(&self) {
		// FIXME: Initialize basic types: [Ljdk/internal/vm/FillerArray;, [Z, [C, [F, [D, [B, [S, [I, [J, [java/lang/Object;
	}

	fn setup_logging() {
		let fixed_window_roller = FixedWindowRoller::builder().build(".log/instructions.{}.log", 0).unwrap();
		let trigger = RolloverOnStartTrigger {};
		let compound_policy = CompoundPolicy::new(Box::new(trigger),Box::new(fixed_window_roller));
		let config = Config::builder()
			.appender(
				Appender::builder()
					.build(
						"instructions",
						Box::new(
							RollingFileAppender::builder()
								.encoder(Box::new(PatternEncoder::new("[{d(%H:%M:%S)} - {h({l})} - {T} (({i})) - {L}@{M}]: {m}{n}")))
								.build(".log/instructions.log", Box::new(compound_policy)).unwrap(),
						),
					),
			)
			.build(
				Root::builder()
					.appender("instructions")
					.build(LevelFilter::Trace),
			).unwrap();
		log4rs::init_config(config).unwrap();
	}
}

#[derive(Debug)]
struct RolloverOnStartTrigger {}

static mut FIRST_CALL: bool = true;

impl Trigger for RolloverOnStartTrigger {
	fn trigger(&self, _: &LogFile<'_>) -> Result<bool> {
		unsafe {
			if FIRST_CALL {
				FIRST_CALL = false;
				Ok(true)
			} else {
				Ok(false)
			}
		}
	}
}

use log4rs::Config;
use log4rs::config::Appender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::Root;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;
use log4rs::append::rolling_file::LogFile;
use anyhow::Result;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;