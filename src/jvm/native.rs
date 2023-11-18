use super::{
    frame::Frame,
    instructions::{
        control::x_return::{areturn, dreturn, freturn, ireturn, lreturn},
        InstructionResult, ReturnKind,
    },
    types::Types,
};

pub mod java;
pub mod jdk;

pub fn native_call(execution_context: &mut Frame) -> InstructionResult {
    // NOTE: Native calls return without invoking any return instruction so we need to manually call them.
    let result = match () {
        _ if execution_context.object_name().starts_with("java/") => {
            java::native_call_java(execution_context)
        }
        _ if execution_context.object_name().starts_with("jdk/") => {
            jdk::native_call_jdk(execution_context)
        }
        _ => panic!("Unknown native class: {}", execution_context.object_name()),
    };
    if let Some(kind) = &result.get_ret() {
        match kind {
            ReturnKind::Value(t) => match t {
                Types::Int(value) => ireturn(value.clone(), execution_context.return_value()),
                Types::Long(value) => lreturn(value.clone(), execution_context.return_value()),
                Types::Float(value) => freturn(value.clone(), execution_context.return_value()),
                Types::Double(value) => dreturn(value.clone(), execution_context.return_value()),
                Types::Reference(value) => areturn(value.clone(), execution_context.return_value()),
                _ => panic!("Expected Int return type"),
            },
            ReturnKind::Void => result,
        }
    } else {
        result
    }
}

pub trait NativeClass {
    fn native_call(
        method_name: &str,
        descriptor: &str,
        execution_context: &mut Frame,
    ) -> InstructionResult;
}
