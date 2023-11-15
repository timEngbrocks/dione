use crate::jvm::{frame::Frame, instructions::InstructionResult, types::object::Object};

pub mod internal;

pub fn native_call_jdk(
    execution_context: &mut Frame,
    object: &Object,
) -> InstructionResult {
    match execution_context.object_name {
        _ if execution_context.object_name.starts_with("jdk/internal") => internal::native_call_jdk_internal(
            execution_context,
            object,
        ),
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
