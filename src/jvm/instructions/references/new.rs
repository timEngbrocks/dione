use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NEW {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for NEW {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::NEW));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		NEW {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!("NEW::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}