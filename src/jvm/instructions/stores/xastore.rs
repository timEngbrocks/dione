use crate::{jvm::{frame::Frame, instructions::{InstructionResult, Instruction}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IASTORE {}
impl Instruction for IASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IASTORE));
		IASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("IASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("iastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LASTORE {}
impl Instruction for LASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LASTORE));
		LASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("LASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("lastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FASTORE {}
impl Instruction for FASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FASTORE));
		FASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("FASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DASTORE {}
impl Instruction for DASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DASTORE));
		DASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("DASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AASTORE {}
impl Instruction for AASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::AASTORE));
		AASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("AASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("aastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BASTORE {}
impl Instruction for BASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::BASTORE));
		BASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("BASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("bastore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CASTORE {}
impl Instruction for CASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::CASTORE));
		CASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("CASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("castore")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SASTORE {}
impl Instruction for SASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::SASTORE));
		SASTORE {}
	}

	fn execute(&self, _: &mut Frame) -> InstructionResult {
		unimplemented!("SASTORE::execute()")
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("sastore")
	}
}