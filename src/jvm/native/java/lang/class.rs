use crate::jvm::{
    frame::Frame,
    instructions::InstructionResult,
    native::NativeClass,
    types::{int::Int, Types, Value},
};

pub struct Class {}

impl NativeClass for Class {
    fn native_call(
        method_name: &str,
        _descriptor: &str,
        execution_context: &mut Frame,
    ) -> InstructionResult {
        match method_name {
            "registerNatives" => Class::register_natives(execution_context),
            "desiredAssertionStatus0" => Class::desired_assertion_status_0(execution_context),
            _ => panic!(
                "Unknown native method: {}.{}",
                "java/lang/Class", method_name
            ),
        }
    }
}

impl Class {
    pub fn register_natives(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::empty()
    }

    pub fn desired_assertion_status_0(_execution_context: &mut Frame) -> InstructionResult {
        // FIXME: Return actual assertion status
        InstructionResult::return_value(Types::Int(Int::from_value(0)))
    }
}
