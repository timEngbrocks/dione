use crate::jvm::{frame::Frame, instructions::InstructionResult};

pub mod misc;

pub fn native_call_jdk_internal(execution_context: &mut Frame) -> InstructionResult {
    match execution_context.object_name {
        _ if execution_context
            .object_name
            .starts_with("jdk/internal/misc") =>
        {
            misc::native_call_jdk_internal_misc(execution_context)
        }
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
