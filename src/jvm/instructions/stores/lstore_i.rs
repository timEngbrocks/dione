use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_0 {}
impl Instruction for LSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSTORE_0));
		LSTORE_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		let value = execution_context.stack.pop();
		match value {
			Types::Long(value) => {
				execution_context.local_variables.set(0, Types::Long(value));
				InstructionResult::empty()
			},
			_ => panic!("LSTORE_0: Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_1 {}
impl Instruction for LSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSTORE_1));
		LSTORE_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		let value = execution_context.stack.pop();
		match value {
			Types::Long(value) => {
				execution_context.local_variables.set(1, Types::Long(value));
				InstructionResult::empty()
			},
			_ => panic!("LSTORE_1: Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_2 {}
impl Instruction for LSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSTORE_2));
		LSTORE_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		let value = execution_context.stack.pop();
		match value {
			Types::Long(value) => {
				execution_context.local_variables.set(2, Types::Long(value));
				InstructionResult::empty()
			},
			_ => panic!("LSTORE_2: Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_3 {}
impl Instruction for LSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSTORE_3));
		LSTORE_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 5);
		let value = execution_context.stack.pop();
		match value {
			Types::Long(value) => {
				execution_context.local_variables.set(3, Types::Long(value));
				InstructionResult::empty()
			},
			_ => panic!("LSTORE_3: Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}
}