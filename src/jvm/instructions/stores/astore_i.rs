use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ASTORE_0 {}
impl Instruction for ASTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ASTORE_0));
		ASTORE_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 1);
		let value = execution_context.stack.pop();
		match value {
			Types::ReturnAddress(value) => {
				execution_context.local_variables.set(0, Types::ReturnAddress(value));
				InstructionResult::empty()
			},
			Types::Reference(value) => {
				execution_context.local_variables.set(0, Types::Reference(value));
				InstructionResult::empty()
			},
			_ => panic!("ASTORE_0: Expected ReturnAddress/Reference"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("astore_0")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ASTORE_1 {}
impl Instruction for ASTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ASTORE_1));
		ASTORE_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		let value = execution_context.stack.pop();
		match value {
			Types::ReturnAddress(value) => {
				execution_context.local_variables.set(1, Types::ReturnAddress(value));
				InstructionResult::empty()
			},
			Types::Reference(value) => {
				execution_context.local_variables.set(0, Types::Reference(value));
				InstructionResult::empty()
			},
			_ => panic!("ASTORE_1: Expected ReturnAddress/Reference"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("astore_1")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ASTORE_2 {}
impl Instruction for ASTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ASTORE_2));
		ASTORE_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		let value = execution_context.stack.pop();
		match value {
			Types::ReturnAddress(value) => {
				execution_context.local_variables.set(2, Types::ReturnAddress(value));
				InstructionResult::empty()
			},
			Types::Reference(value) => {
				execution_context.local_variables.set(0, Types::Reference(value));
				InstructionResult::empty()
			},
			_ => panic!("ASTORE_2: Expected ReturnAddress/Reference"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("astore_2")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ASTORE_3 {}
impl Instruction for ASTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ASTORE_3));
		ASTORE_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		let value = execution_context.stack.pop();
		match value {
			Types::ReturnAddress(value) => {
				execution_context.local_variables.set(3, Types::ReturnAddress(value));
				InstructionResult::empty()
			},
			Types::Reference(value) => {
				execution_context.local_variables.set(0, Types::Reference(value));
				InstructionResult::empty()
			},
			_ => panic!("ASTORE_3: Expected ReturnAddress/Reference"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("astore_3")
	}
}