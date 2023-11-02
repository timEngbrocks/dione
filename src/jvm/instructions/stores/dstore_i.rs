use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSTORE_0 {}
impl Instruction for DSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSTORE_0));
		DSTORE_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		let value = execution_context.stack.pop();
		match value {
			Types::Double(value) => {
				execution_context.local_variables.set(0, Types::Double(value));
				InstructionResult::empty()
			},
			_ => panic!("DSTORE_0: Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dstore_0")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSTORE_1 {}
impl Instruction for DSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSTORE_1));
		DSTORE_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		let value = execution_context.stack.pop();
		match value {
			Types::Double(value) => {
				execution_context.local_variables.set(1, Types::Double(value));
				InstructionResult::empty()
			},
			_ => panic!("DSTORE_1: Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dstore_1")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSTORE_2 {}
impl Instruction for DSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSTORE_2));
		DSTORE_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		let value = execution_context.stack.pop();
		match value {
			Types::Double(value) => {
				execution_context.local_variables.set(2, Types::Double(value));
				InstructionResult::empty()
			},
			_ => panic!("DSTORE_2: Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dstore_2")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSTORE_3 {}
impl Instruction for DSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSTORE_3));
		DSTORE_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 5);
		let value = execution_context.stack.pop();
		match value {
			Types::Double(value) => {
				execution_context.local_variables.set(3, Types::Double(value));
				InstructionResult::empty()
			},
			_ => panic!("DSTORE_3: Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dstore_3")
	}
}