use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{Types, Value, int::Int, long::Long, float::Float, double::Double}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IDIV {}
impl Instruction for IDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IDIV));
		IDIV {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 / value2;
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
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LDIV {}
impl Instruction for LDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LDIV));
		LDIV {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value2) => {
				match execution_context.stack.pop() {
					Types::Long(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 / value2;
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
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FDIV {}
impl Instruction for FDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FDIV));
		FDIV {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value2) => {
				match execution_context.stack.pop() {
					Types::Float(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 / value2;
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
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DDIV {}
impl Instruction for DDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DDIV));
		DDIV {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value2) => {
				match execution_context.stack.pop() {
					Types::Double(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = value1 / value2;
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
}