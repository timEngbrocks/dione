use super::{
    frame::Frame,
    instructions::{
        control::x_return::{areturn, dreturn, freturn, ireturn, lreturn},
        InstructionResult, ReturnKind,
    },
    types::{object::Object, Types},
};

pub mod java;

pub fn native_call(
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    execution_context: &mut Frame,
    object: &Object,
) -> InstructionResult {
    // NOTE: Native calls return without invoking any return instruction so we need to manually call them.
    let result = match class_name {
        _ if class_name.starts_with("java/") => java::native_call_java(
            class_name,
            method_name,
            descriptor,
            execution_context,
            object,
        ),
        _ => panic!("Unknown native class: {}", class_name),
    };
    if let Some(kind) = &result.ret {
        match kind {
            ReturnKind::Value(t) => match t {
                Types::Int(value) => ireturn(value.clone(), &execution_context.return_value),
                Types::Long(value) => lreturn(value.clone(), &execution_context.return_value),
                Types::Float(value) => freturn(value.clone(), &execution_context.return_value),
                Types::Double(value) => dreturn(value.clone(), &execution_context.return_value),
                Types::Reference(value) => areturn(value.clone(), &execution_context.return_value),
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
        object: &Object,
    ) -> InstructionResult;
}
