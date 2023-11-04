use crate::jvm::{
    frame::Frame, instructions::InstructionResult, native::NativeClass, types::object::Object,
};

pub struct System {}

impl NativeClass for System {
    fn native_call(
        method_name: &str,
        _descriptor: &str,
        execution_context: &mut Frame,
        object: &Object,
    ) -> InstructionResult {
        match method_name {
            "registerNatives" => System::register_natives(execution_context, object),
            _ => panic!(
                "Unknown native method: {}.{}",
                "java/lang/System", method_name
            ),
        }
    }
}

impl System {
    pub fn register_natives(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
        InstructionResult::empty()
    }
}
