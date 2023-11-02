use crate::{jvm::{instructions::{Instruction, InstructionResult}, frame::Frame, types::Types}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD {
	index: U1,
}
impl Instruction for ILOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ILOAD));
		let index = parser.consume_u1();
		ILOAD {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= self.index as u16);
		match execution_context.local_variables.get(self.index as u16) {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(value.clone()));
			},
			_ => panic!("ILOAD: Expected a Int")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("iload {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD {
	index: U1,
}
impl Instruction for LLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LLOAD));
		let index = parser.consume_u1();
		LLOAD {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() > self.index as u16);
		match execution_context.local_variables.get(self.index as u16) {
			Types::Long(value) => {
				execution_context.stack.push(Types::Long(value.clone()));
			},
			_ => panic!("LLOAD: Expected a Long")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("lload {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD {
	index: U1,
}
impl Instruction for FLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FLOAD));
		let index = parser.consume_u1();
		FLOAD {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= self.index as u16);
		match execution_context.local_variables.get(self.index as u16) {
			Types::Float(value) => {
				execution_context.stack.push(Types::Float(value.clone()));
			},
			_ => panic!("FLOAD: Expected a Float")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("fload {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD {
	index: U1,
}
impl Instruction for DLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DLOAD));
		let index = parser.consume_u1();
		DLOAD {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() > self.index as u16);
		match execution_context.local_variables.get(self.index as u16) {
			Types::Double(value) => {
				execution_context.stack.push(Types::Double(value.clone()));
			},
			_ => panic!("DLOAD: Expected a Double")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("dload {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ALOAD {
	index: U1,
}
impl Instruction for ALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ALOAD));
		let index = parser.consume_u1();
		ALOAD {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(execution_context.local_variables.len() >= self.index as u16);
		match execution_context.local_variables.get(self.index as u16) {
			Types::ReturnAddress(value) => {
				execution_context.stack.push(Types::ReturnAddress(value.clone()));
			},
			Types::Reference(value) => {
				execution_context.stack.push(Types::Reference(value.clone()));
			},
			_ => panic!("ALOAD: Expected a ReturnAddress/Reference")
		}
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("aload {}", self.index)
	}
}