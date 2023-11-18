use crate::util::heap::Heap;

use self::{interpreter::Interpreter, object_manager::ObjectManager};

pub mod bootstrap_class_loader;
pub mod descriptor;
pub mod exception_handler;
pub mod execution_context;
pub mod frame;
pub mod instructions;
pub mod interpreter;
pub mod logging;
pub mod native;
pub mod object_manager;
pub mod runtime_constant_pool;
pub mod types;

static mut INSTANCE: Option<JVM> = None;

pub struct JVM {
    interpreter: Interpreter,
}

impl JVM {
    pub fn start(jdk_base_path: String) {
        logging::setup_logging();

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

        self.call_system_init_phase2();
        self.call_system_init_phase3();

        // FIXME: SystemDictonary::compute_java_loaders

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

    fn initialize_java_lang_classes(&mut self) {
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

        self.call_system_init_phase1();

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

    fn genesis(&self) {
        // FIXME: Initialize basic types: [Ljdk/internal/vm/FillerArray;, [Z, [C, [F, [D, [B, [S, [I, [J, [java/lang/Object;
    }

    fn call_system_init_phase1(&mut self) {
        let mut temporary_interpreter = Interpreter::new();
        temporary_interpreter.run("java/lang/System", "initPhase1", "()V");
    }

    fn call_system_init_phase2(&mut self) {
        let mut temporary_interpreter = Interpreter::new();
        temporary_interpreter.run("java/lang/System", "initPhase2", "(ZZ)I");
    }

    fn call_system_init_phase3(&mut self) {
        let mut temporary_interpreter = Interpreter::new();
        temporary_interpreter.run("java/lang/System", "initPhase3", "()V");
    }
}
