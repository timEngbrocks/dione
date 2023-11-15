use crate::jvm::{frame::Frame, instructions::InstructionResult, types::object::Object};

pub mod misc;

pub fn native_call_jdk_internal(
    execution_context: &mut Frame,
    object: &Object,
) -> InstructionResult {
    match execution_context.object_name {
        _ if execution_context.object_name.starts_with("jdk/internal/misc") => misc::native_call_jdk_internal_misc(
            execution_context,
            object,
        ),
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
