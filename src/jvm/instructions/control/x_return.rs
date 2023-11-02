use crate::{jvm::{instructions::{Instruction, InstructionResult, conversions::x2y::{i2b, i2c, i2s}}, frame::Frame, types::{Types, ReturnTypes, boolean::{Boolean, BooleanValue}, Value}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IRETURN {}
impl Instruction for IRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IRETURN));
		IRETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				let return_value = match &execution_context.return_value {
					ReturnTypes::Type(t) => match t {
						Types::Byte(_) => Types::Byte(i2b(value)),
						Types::Char(_) => Types::Char(i2c(value)),
						Types::Short(_) => Types::Short(i2s(value)),
						Types::Boolean(_) => Types::Boolean(Boolean::from_value(BooleanValue::from_value(value.get() & 1))),
						Types::Int(_) => Types::Int(value),
						_ => panic!("Expected Int return type"),
					},
					ReturnTypes::Void => panic!("Expected Int return type"),
				};
				InstructionResult::return_value(return_value)
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("ireturn")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LRETURN {}
impl Instruction for LRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LRETURN));
		LRETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value) => InstructionResult::return_value(Types::Long(value)),
			_ => panic!("Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("lreturn")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FRETURN {}
impl Instruction for FRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FRETURN));
		FRETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value) => InstructionResult::return_value(Types::Float(value)),
			_ => panic!("Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("freturn")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DRETURN {}
impl Instruction for DRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DRETURN));
		DRETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value) => InstructionResult::return_value(Types::Double(value)),
			_ => panic!("Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("dreturn")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARETURN {}
impl Instruction for ARETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ARETURN));
		ARETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			// FIXME: Check that the reference on the stack and the return type are assignment compatible
			Types::Reference(value) => InstructionResult::return_value(Types::Reference(value)),
			_ => panic!("Expected Reference"),
		}
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("areturn")
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RETURN {}
impl Instruction for RETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::RETURN));
		RETURN {}
	}

	// TODO: Monitor update
	// TODO: Exception handling
	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		InstructionResult::return_void()
	}

	fn length(&self) -> U2 {
		1
	}

	fn to_string(&self) -> String {
		String::from("return")
	}
}