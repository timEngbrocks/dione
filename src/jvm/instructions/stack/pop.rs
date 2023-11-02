use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::ComputationalTypeCategory}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct POP {}
impl Instruction for POP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::POP));
		POP {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop().get_computational_type_category() {
			ComputationalTypeCategory::Type1 => {
				InstructionResult::empty()
			},
			ComputationalTypeCategory::Type2 => panic!("POP: Expected a Type1 value")
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("pop")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct POP2 {}
impl Instruction for POP2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::POP2));
		POP2 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop().get_computational_type_category() {
			ComputationalTypeCategory::Type1 => {
				execution_context.stack.pop();
				InstructionResult::empty()
			},
			ComputationalTypeCategory::Type2 => InstructionResult::empty()
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("pop2")
	}
}