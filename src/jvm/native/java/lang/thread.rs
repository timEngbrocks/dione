use crate::jvm::{frame::Frame, instructions::InstructionResult, native::NativeClass};

pub struct Thread {}

impl NativeClass for Thread {
    fn native_call(
        method_name: &str,
        _descriptor: &str,
        execution_context: &mut Frame,
    ) -> InstructionResult {
        match method_name {
            "registerNatives" => Thread::register_natives(execution_context),
            _ => panic!(
                "Unknown native method: {}.{}",
                "java/lang/Thread", method_name
            ),
        }
    }
}

impl Thread {
    pub fn register_natives(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::empty()
    }
}
