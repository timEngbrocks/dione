use crate::{jvm::{instructions::{Instruction, Instructions, InstructionResult}, frame::Frame}, class_loader::parser::{Parser, U2}};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NOP {}

impl Instruction for NOP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::NOP as u8));
		NOP {}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		1
	}
}