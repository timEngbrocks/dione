use crate::{jvm::{frame::Frame, types::{int::Int, Value, Types, long::Long}, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISHR {}
impl Instruction for ISHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ISHR));
		ISHR {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let s = value2.get() & 0b11111;
						execution_context.stack.push(Types::Int(Int::from_value(value1.get() >> s)));
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
pub struct LSHR {}
impl Instruction for LSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSHR));
		LSHR {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let s = value2.get() & 0b111111;
						execution_context.stack.push(Types::Long(Long::from_value((value1.get() as i64) >> s as i64)));
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