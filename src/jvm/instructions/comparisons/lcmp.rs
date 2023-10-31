use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{Types, Value, int::Int}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LCMP {}
impl Instruction for LCMP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LCMP));
		LCMP {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value2) => {
				match execution_context.stack.pop() {
					Types::Long(value1) => {
						let value1 = value1.get();
						let value2 = value2.get();
						let result = if value1 > value2 {
							1
						} else if value1 == value2 {
							0
						} else {
							-1
						};
						execution_context.stack.push(Types::Int(Int::from_value(result)));
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