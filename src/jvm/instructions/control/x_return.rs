use crate::{jvm::{instructions::Instruction, frame::Frame}, class_loader::parser::{Parser, U2}, opcodes};

#[allow(non_camel_case_types)]
pub struct IRETURN {}
impl Instruction for IRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IRETURN));
		IRETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("IRETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LRETURN {}
impl Instruction for LRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LRETURN));
		LRETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("LRETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FRETURN {}
impl Instruction for FRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FRETURN));
		FRETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("FRETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DRETURN {}
impl Instruction for DRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DRETURN));
		DRETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("DRETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ARETURN {}
impl Instruction for ARETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ARETURN));
		ARETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("ARETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct RETURN {}
impl Instruction for RETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::RETURN));
		RETURN {}
	}

	fn execute(&mut self, _: &mut Frame) {
		unimplemented!("RETURN::execute")
	}

	fn length(&self) -> U2 {
		1
	}
}