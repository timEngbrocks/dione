use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ATHROW {}
impl Instruction for ATHROW {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ATHROW));
		ATHROW {}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("athrow")
	}
}