use crate::{jvm::{instructions::{Instruction, InstructionResult}, frame::Frame, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD_0 {}
impl Instruction for ILOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ILOAD_0));
		ILOAD_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 1);
		match execution_context.local_variables.get(0) {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(value.clone()));
			},
			_ => panic!("ILOAD_0: Expected a Int")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD_1 {}
impl Instruction for ILOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ILOAD_1));
		ILOAD_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		match execution_context.local_variables.get(1) {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(value.clone()));
			},
			_ => panic!("ILOAD_1: Expected a Int")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}
impl Instruction for ILOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ILOAD_2));
		ILOAD_2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		match execution_context.local_variables.get(2) {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(value.clone()));
			},
			_ => panic!("ILOAD_2: Expected a Int")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD_3 {}
impl Instruction for ILOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ILOAD_3));
		ILOAD_3 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		match execution_context.local_variables.get(3) {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(value.clone()));
			},
			_ => panic!("ILOAD_3: Expected a Int")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}