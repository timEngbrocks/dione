use crate::{jvm::{instructions::{Instruction, InstructionResult}, frame::Frame, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_0 {}
impl Instruction for LLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LLOAD_0));
		LLOAD_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.stack.len() >= 2);
		match execution_context.local_variables.get(0) {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(value.clone()));
			},
			_ => panic!("LLOAD_0: Expected a Long")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_1 {}
impl Instruction for LLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LLOAD_1));
		LLOAD_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.stack.len() >= 3);
		match execution_context.local_variables.get(1) {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(value.clone()));
			},
			_ => panic!("LLOAD_1: Expected a Long")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_2 {}
impl Instruction for LLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LLOAD_2));
		LLOAD_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.stack.len() >= 4);
		match execution_context.local_variables.get(2) {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(value.clone()));
			},
			_ => panic!("LLOAD_2: Expected a Long")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_3 {}
impl Instruction for LLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LLOAD_3));
		LLOAD_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.stack.len() >= 5);
		match execution_context.local_variables.get(3) {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(value.clone()));
			},
			_ => panic!("LLOAD_3: Expected a Long")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}