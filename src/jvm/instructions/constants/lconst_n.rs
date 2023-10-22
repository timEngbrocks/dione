use crate::{jvm::{frame::Frame, instructions::Instruction, types::{long::Long, Value, Types}}, class_loader::parser::{Parser, U2}, opcodes};

#[allow(non_camel_case_types)]
pub struct LCONST_0 {}
impl Instruction for LCONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LCONST_0));
		LCONST_0 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		let value = Long::from_value(0);
		execution_context.stack.push(Types::Long(value));
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LCONST_1 {}
impl Instruction for LCONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LCONST_1));
		LCONST_1 {}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		let value = Long::from_value(1);
		execution_context.stack.push(Types::Long(value));
	}

	fn length(&self) -> U2 {
		1
	}
}