use crate::{jvm::{instructions::{Instruction, Instructions}, execution_context::ExecutionContext}, class_loader::parser::{Parser, U2}};

pub struct NOP {}

impl Instruction for NOP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::NOP as u8));
		NOP {}
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}