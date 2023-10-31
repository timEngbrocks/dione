use crate::{jvm::{instructions::{Instruction, InstructionResult}, frame::Frame, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_0 {}
impl Instruction for FLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FLOAD_0));
		FLOAD_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 1);
		match execution_context.local_variables.get(0) {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(value.clone()));
			},
			_ => panic!("FLOAD_0: Expected a Float")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_1 {}
impl Instruction for FLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FLOAD_1));
		FLOAD_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		match execution_context.local_variables.get(1) {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(value.clone()));
			},
			_ => panic!("FLOAD_1: Expected a Float")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_2 {}
impl Instruction for FLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FLOAD_2));
		FLOAD_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		match execution_context.local_variables.get(2) {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(value.clone()));
			},
			_ => panic!("FLOAD_2: Expected a Float")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_3 {}
impl Instruction for FLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FLOAD_3));
		FLOAD_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		match execution_context.local_variables.get(3) {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(value.clone()));
			},
			_ => panic!("FLOAD_3: Expected a Float")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}