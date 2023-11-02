use super::{frame::Frame, instructions::InstructionResult, types::object::Object};

pub mod java;

pub fn native_call(class_name: &str, method_name: &str, descriptor: &str, execution_context: &mut Frame, object: &Object) -> InstructionResult {
	match class_name {
		_ if class_name.starts_with("java/") => java::native_call_java(class_name, method_name, descriptor, execution_context, object),
		_ => panic!("Unknown native class: {}", class_name),
	}
}

pub trait NativeClass {
	fn native_call(method_name: &str, descriptor: &str, execution_context: &mut Frame, object: &Object) -> InstructionResult;
}