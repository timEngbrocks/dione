use crate::jvm::types::object::Object;
use crate::jvm::{frame::Frame, instructions::InstructionResult};

use crate::jvm::native::NativeClass;

use self::class::Class;
use self::system::System;
use self::thread::Thread;

pub mod class;
pub mod system;
pub mod thread;

pub fn native_call_java_lang(
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    execution_context: &mut Frame,
    object: &Object,
) -> InstructionResult {
    match class_name {
        "java/lang/System" => {
            System::native_call(method_name, descriptor, execution_context, object)
        }
        "java/lang/Class" => Class::native_call(method_name, descriptor, execution_context, object),
        "java/lang/Thread" => {
            Thread::native_call(method_name, descriptor, execution_context, object)
        }
        _ => panic!("Unknown native class: {}", class_name),
    }
}
