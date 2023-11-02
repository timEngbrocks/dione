use crate::jvm::{native::NativeClass, frame::Frame, instructions::InstructionResult, types::object::Object};

pub struct Thread {}

impl NativeClass for Thread {
    fn native_call(method_name: &str, _descriptor: &str, execution_context: &mut Frame, object: &Object) -> InstructionResult {
        match method_name {
            "registerNatives" => Thread::register_natives(execution_context, object),
            _ => panic!("Unknown native method: {}.{}", "java/lang/Thread", method_name),
        }
    }
}

impl Thread {
    pub fn register_natives(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
        InstructionResult::empty()
    }
}