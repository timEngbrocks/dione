use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{reference::Reference, Types, Value}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ACONST_NULL {}
impl Instruction for ACONST_NULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ACONST_NULL));
		ACONST_NULL {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let value = Reference::new();
		execution_context.stack.push(Types::Reference(value));
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}