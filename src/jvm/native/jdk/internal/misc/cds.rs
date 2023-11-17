use crate::jvm::{
    frame::Frame,
    instructions::InstructionResult,
    native::NativeClass,
    types::{int::Int, Types, Value},
};

pub struct CDS {}

impl NativeClass for CDS {
    fn native_call(
        method_name: &str,
        _descriptor: &str,
        execution_context: &mut Frame,
    ) -> InstructionResult {
        match method_name {
            "registerNatives" => CDS::register_natives(execution_context),
            "isDumpingClassList0" => CDS::is_dumping_class_list0(execution_context),
            "isDumpingArchive0" => CDS::is_dumping_archive0(execution_context),
            "isSharingEnabled0" => CDS::is_sharing_enabled0(execution_context),
            "initializeFromArchive" => CDS::initialize_from_archive(execution_context),
            _ => panic!(
                "Unknown native method: {}.{}",
                "jdk/internal/misc/CDS", method_name
            ),
        }
    }
}

impl CDS {
    pub fn register_natives(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::empty()
    }

    pub fn is_dumping_class_list0(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::return_value(Types::Int(Int::from_value(0)))
    }

    pub fn is_dumping_archive0(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::return_value(Types::Int(Int::from_value(0)))
    }

    pub fn is_sharing_enabled0(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::return_value(Types::Int(Int::from_value(0)))
    }

    pub fn initialize_from_archive(_execution_context: &mut Frame) -> InstructionResult {
        InstructionResult::empty()
    }
}
