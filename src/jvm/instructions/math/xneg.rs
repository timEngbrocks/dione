use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{Types, Value, int::Int, long::Long, float::Float, double::Double}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INEG {}
impl Instruction for INEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INEG));
		INEG {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(Int::from_value(-value.get())));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("ineg")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LNEG {}
impl Instruction for LNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LNEG));
		LNEG {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(Long::from_value(-value.get())));
				InstructionResult::empty()
			},
			_ => panic!("Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("lneg")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FNEG {}
impl Instruction for FNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FNEG));
		FNEG {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(Float::from_value(-value.get())));
				InstructionResult::empty()
			},
			_ => panic!("Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("fneg")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DNEG {}
impl Instruction for DNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DNEG));
		DNEG {}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(Double::from_value(-value.get())));
				InstructionResult::empty()
			},
			_ => panic!("Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dneg")
	}
}