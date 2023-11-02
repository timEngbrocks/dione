use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSTORE_0 {}
impl Instruction for FSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSTORE_0));
		FSTORE_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 1);
		let value = execution_context.stack.pop();
		match value {
			Types::Float(value) => {
				execution_context.local_variables.set(0, Types::Float(value));
				InstructionResult::empty()
			},
			_ => panic!("FSTORE_0: Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fstore_0")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSTORE_1 {}
impl Instruction for FSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSTORE_1));
		FSTORE_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		let value = execution_context.stack.pop();
		match value {
			Types::Float(value) => {
				execution_context.local_variables.set(1, Types::Float(value));
				InstructionResult::empty()
			},
			_ => panic!("FSTORE_1: Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fstore_1")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSTORE_2 {}
impl Instruction for FSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSTORE_2));
		FSTORE_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		let value = execution_context.stack.pop();
		match value {
			Types::Float(value) => {
				execution_context.local_variables.set(2, Types::Float(value));
				InstructionResult::empty()
			},
			_ => panic!("FSTORE_2: Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fstore_2")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSTORE_3 {}
impl Instruction for FSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSTORE_3));
		FSTORE_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		let value = execution_context.stack.pop();
		match value {
			Types::Float(value) => {
				execution_context.local_variables.set(3, Types::Float(value));
				InstructionResult::empty()
			},
			_ => panic!("FSTORE_3: Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fstore_3")
	}
}