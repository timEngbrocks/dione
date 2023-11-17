use crate::jvm::{frame::Frame, instructions::InstructionResult};

use crate::jvm::native::NativeClass;

use self::class::Class;
use self::system::System;
use self::thread::Thread;

pub mod class;
pub mod system;
pub mod thread;

pub fn native_call_java_lang(execution_context: &mut Frame) -> InstructionResult {
    match execution_context.object_name.as_str() {
        "java/lang/System" => System::native_call(
            &execution_context.method_name.clone(),
            &execution_context.descriptor.clone(),
            execution_context,
        ),
        "java/lang/Class" => Class::native_call(
            &execution_context.method_name.clone(),
            &execution_context.descriptor.clone(),
            execution_context,
        ),
        "java/lang/Thread" => Thread::native_call(
            &execution_context.method_name.clone(),
            &execution_context.descriptor.clone(),
            execution_context,
        ),
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
