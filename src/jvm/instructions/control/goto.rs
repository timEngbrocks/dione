use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GOTO {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for GOTO {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::GOTO));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		GOTO {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!("GOTO::execute")
	}

	fn length(&self) -> U2 {
		3
	}
}