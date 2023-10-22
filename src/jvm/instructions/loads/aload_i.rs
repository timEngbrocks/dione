use crate::{jvm::{instructions::Instruction, frame::Frame}, class_loader::parser::{Parser, U2}, opcodes};

#[allow(non_camel_case_types)]
pub struct ALOAD_0 {}
impl Instruction for ALOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ALOAD_0));
		ALOAD_0 {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("ALOAD_0::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_1 {}
impl Instruction for ALOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ALOAD_1));
		ALOAD_1 {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("ALOAD_1::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_2 {}
impl Instruction for ALOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ALOAD_2));
		ALOAD_2 {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("ALOAD_2::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_3 {}
impl Instruction for ALOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ALOAD_3));
		ALOAD_3 {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("ALOAD_3::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}