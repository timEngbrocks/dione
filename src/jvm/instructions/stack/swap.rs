use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::ComputationalTypeCategory}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SWAP {}
impl Instruction for SWAP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::SWAP));
		SWAP {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			value1 if value1.get_computational_type_category() == ComputationalTypeCategory::Type1 => {
				match execution_context.stack.pop() {
					value2 if value2.get_computational_type_category() == ComputationalTypeCategory::Type1 => {
						execution_context.stack.push(value1);
						execution_context.stack.push(value2);
						InstructionResult::empty()
					},
					_ => panic!("SWAP: Expected a Type1 value as the second value")
				}
			},
			_ => panic!("SWAP: Expected a Type1 value as the first value")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}