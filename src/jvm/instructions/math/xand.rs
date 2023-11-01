use crate::{jvm::{frame::Frame, types::{int::Int, Value, Types, long::Long}, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IAND {}
impl Instruction for IAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IAND));
		IAND {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						execution_context.stack.push(Types::Int(Int::from_value(value1.get() & value2.get())));
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
pub struct LAND {}
impl Instruction for LAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LAND));
		LAND {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value2) => {
				match execution_context.stack.pop() {
					Types::Long(value1) => {
						execution_context.stack.push(Types::Long(Long::from_value(value1.get() & value2.get())));
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