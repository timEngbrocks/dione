use crate::{jvm::{frame::Frame, instructions::Instruction}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[allow(non_camel_case_types)]
pub struct INVOKESPECIAL {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for INVOKESPECIAL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKESPECIAL));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		INVOKESPECIAL {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) {
		let _ = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;

	}

	fn length(&self) -> U2 {
		3
	}
}