use crate::jvm::{
    frame::Frame,
    instructions::InstructionResult,
    native::NativeClass,
    types::{object::Object, int::Int, Value, Types},
};

pub struct CDS {}

impl NativeClass for CDS {
    fn native_call(
        method_name: &str,
        _descriptor: &str,
        execution_context: &mut Frame,
        object: &Object,
    ) -> InstructionResult {
        match method_name {
            "registerNatives" => CDS::register_natives(execution_context, object),
			"isDumpingClassList0" => CDS::is_dumping_class_list0(execution_context, object),
			"isDumpingArchive0" => CDS::is_dumping_archive0(execution_context, object),
			"isSharingEnabled0" => CDS::is_sharing_enabled0(execution_context, object),
			"initializeFromArchive" => CDS::initialize_from_archive(execution_context, object),
            _ => panic!(
                "Unknown native method: {}.{}",
                "jdk/internal/misc/CDS", method_name
            ),
        }
    }
}

impl CDS {
    pub fn register_natives(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
        InstructionResult::empty()
    }

	pub fn is_dumping_class_list0(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
		InstructionResult::return_value(Types::Int(Int::from_value(0)))
	}

	pub fn is_dumping_archive0(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
		InstructionResult::return_value(Types::Int(Int::from_value(0)))
	}

	pub fn is_sharing_enabled0(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
		InstructionResult::return_value(Types::Int(Int::from_value(0)))
	}

	pub fn initialize_from_archive(_execution_context: &mut Frame, _object: &Object) -> InstructionResult {
		InstructionResult::empty()
	}
}
