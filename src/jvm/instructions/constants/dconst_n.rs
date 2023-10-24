use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{double::Double, Value, Types}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DCONST_0 {}
impl Instruction for DCONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DCONST_0));
		DCONST_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let value = Double::from_value(0.0);
		execution_context.stack.push(Types::Double(value));
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DCONST_1 {}
impl Instruction for DCONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DCONST_1));
		DCONST_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let value = Double::from_value(1.0);
		execution_context.stack.push(Types::Double(value));
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}