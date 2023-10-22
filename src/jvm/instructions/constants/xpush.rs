use crate::{jvm::{frame::Frame, instructions::Instruction, types::{Value, int::Int, Types}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[allow(non_camel_case_types)]
pub struct BIPUSH {
	byte: U1,
}
impl Instruction for BIPUSH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::BIPUSH));
		let byte = parser.consume_u1();
		BIPUSH {
			byte,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		let value = Int::from_value(bitutils::sign_extend32(self.byte as u32, 8));
		execution_context.stack.push(Types::Int(value));
	}

	fn length(&self) -> U2 {
		2
	}
}
#[allow(non_camel_case_types)]
pub struct SIPUSH {
	byte1: U1,
	byte2: U1,
}
impl Instruction for SIPUSH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::SIPUSH));
		let byte1 = parser.consume_u1();
		let byte2 = parser.consume_u1();
		SIPUSH {
			byte1,
			byte2,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		let intermediate_short = ((self.byte1 as u16) << 8) | self.byte2 as u16;
		let value = Int::from_value(bitutils::sign_extend32(intermediate_short as u32, 16));
		execution_context.stack.push(Types::Int(value));
	}

	fn length(&self) -> U2 {
		3
	}
}