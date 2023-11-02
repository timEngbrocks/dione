use crate::jvm::{frame::Frame, instructions::InstructionResult, types::object::Object};

pub mod lang;

pub fn native_call_java(class_name: &str, method_name: &str, descriptor: &str, execution_context: &mut Frame, object: &Object) -> InstructionResult {
	match class_name {
		_ if class_name.starts_with("java/lang/") => lang::native_call_java_lang(class_name, method_name, descriptor, execution_context, object),
		_ => panic!("Unknown native class: {}", class_name),
	}
}