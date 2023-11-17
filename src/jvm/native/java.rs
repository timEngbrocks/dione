use crate::jvm::{frame::Frame, instructions::InstructionResult};

pub mod lang;

pub fn native_call_java(execution_context: &mut Frame) -> InstructionResult {
    match execution_context.object_name {
        _ if execution_context.object_name.starts_with("java/lang/") => {
            lang::native_call_java_lang(execution_context)
        }
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
