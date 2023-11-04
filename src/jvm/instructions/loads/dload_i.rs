use crate::{jvm::{instructions::{Instruction, InstructionResult}, frame::Frame, types::Types}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_0 {}
impl Instruction for DLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DLOAD_0));
		DLOAD_0 {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 2);
		match execution_context.local_variables.get(0) {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(value.clone()));
			},
			_ => panic!("DLOAD_0: Expected a Double")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dload_0")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_1 {}
impl Instruction for DLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DLOAD_1));
		DLOAD_1 {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 3);
		match execution_context.local_variables.get(1) {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(value.clone()));
			},
			_ => panic!("DLOAD_1: Expected a Double")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dload_1")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_2 {}
impl Instruction for DLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DLOAD_2));
		DLOAD_2 {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 4);
		match execution_context.local_variables.get(2) {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(value.clone()));
			},
			_ => panic!("DLOAD_2: Expected a Double")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dload_2")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_3 {}
impl Instruction for DLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DLOAD_3));
		DLOAD_3 {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= 5);
		match execution_context.local_variables.get(3) {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(value.clone()));
			},
			_ => panic!("DLOAD_3: Expected a Double")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dload_3")
	}
}