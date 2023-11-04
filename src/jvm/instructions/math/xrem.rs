use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{Types, Value, int::Int, long::Long, float::Float, double::Double}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IREM {}
impl Instruction for IREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IREM));
		IREM {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 - (value1 / value2) * value2;
						execution_context.stack.push(Types::Int(Int::from_value(result)));
						InstructionResult::empty()
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("irem")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LREM {}
impl Instruction for LREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LREM));
		LREM {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value2) => {
				match execution_context.stack.pop() {
					Types::Long(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 - (value1 / value2) * value2;
						execution_context.stack.push(Types::Long(Long::from_value(result)));
						InstructionResult::empty()
					},
					_ => panic!("Expected Long"),
				}
			},
			_ => panic!("Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("lrem")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FREM {}
impl Instruction for FREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FREM));
		FREM {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value2) => {
				match execution_context.stack.pop() {
					Types::Float(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 - (value1 / value2) * value2;
						execution_context.stack.push(Types::Float(Float::from_value(result)));
						InstructionResult::empty()
					},
					_ => panic!("Expected Float"),
				}
			},
			_ => panic!("Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("frem")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DREM {}
impl Instruction for DREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DREM));
		DREM {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value2) => {
				match execution_context.stack.pop() {
					Types::Double(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 - (value1 / value2) * value2;
						execution_context.stack.push(Types::Double(Double::from_value(result)));
						InstructionResult::empty()
					},
					_ => panic!("Expected Double"),
				}
			},
			_ => panic!("Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("drem")
	}
}