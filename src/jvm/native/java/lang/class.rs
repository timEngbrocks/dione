use crate::jvm::{native::NativeClass, frame::Frame, instructions::InstructionResult, types::object::Object};

pub struct Class {}

impl NativeClass for Class {
    fn native_call(method_name: &str, _descriptor: &str, execution_context: &mut Frame, object: &Object) -> InstructionResult {
        match method_name {
            "registerNatives" => Class::register_natives(execution_context, object),
            _ => panic!("Unknown native method: {}.{}", "java/lang/Class", method_name),
        }
    }
}

impl Class {
    pub fn register_natives(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
        InstructionResult::empty()
    }
}