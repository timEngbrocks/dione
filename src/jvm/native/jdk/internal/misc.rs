use crate::jvm::{frame::Frame, instructions::InstructionResult};

use crate::jvm::native::NativeClass;

use self::cds::CDS;

pub mod cds;

pub fn native_call_jdk_internal_misc(execution_context: &mut Frame) -> InstructionResult {
    match execution_context.object_name.as_str() {
        "jdk/internal/misc/CDS" => CDS::native_call(
            &execution_context.method_name.clone(),
            &execution_context.descriptor.clone(),
            execution_context,
        ),
        _ => panic!("Unknown native class: {}", execution_context.object_name),
    }
}
