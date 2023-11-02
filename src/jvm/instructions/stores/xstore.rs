use crate::{jvm::{frame::Frame, types::Types, instructions::{InstructionResult, Instruction}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISTORE {
	index: U1,
}
impl Instruction for ISTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ISTORE));
		let index = parser.consume_u1();
		ISTORE {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!((self.index as u16) < execution_context.local_variables.len());
		match execution_context.stack.pop() {
			Types::Int(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Int(value));
				InstructionResult::empty()
			},
			_ => panic!("ISTORE: Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("istore {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE {
	index: U1,
}
impl Instruction for LSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSTORE));
		let index = parser.consume_u1();
		LSTORE {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!((self.index as u16) < execution_context.local_variables.len());
		match execution_context.stack.pop() {
			Types::Long(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Long(value));
				InstructionResult::empty()
			},
			_ => panic!("LSTORE: Expected Long"),
		}
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("lstore {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSTORE {
	index: U1,
}
impl Instruction for FSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSTORE));
		let index = parser.consume_u1();
		FSTORE {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!((self.index as u16) < execution_context.local_variables.len());
		match execution_context.stack.pop() {
			Types::Float(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Float(value));
				InstructionResult::empty()
			},
			_ => panic!("FSTORE: Expected Float"),
		}
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("fstore {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSTORE {
	index: U1,
}
impl Instruction for DSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSTORE));
		let index = parser.consume_u1();
		DSTORE {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!((self.index as u16) < execution_context.local_variables.len());
		match execution_context.stack.pop() {
			Types::Double(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Double(value));
				InstructionResult::empty()
			},
			_ => panic!("DSTORE: Expected Double"),
		}
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("dstore {}", self.index)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ASTORE {
	index: U1,
}
impl Instruction for ASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ASTORE));
		let index = parser.consume_u1();
		ASTORE {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!((self.index as u16) < execution_context.local_variables.len());
		match execution_context.stack.pop() {
			Types::ReturnAddress(value) => {
				execution_context.local_variables.set(self.index as u16, Types::ReturnAddress(value));
				InstructionResult::empty()
			},
			Types::Reference(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Reference(value));
				InstructionResult::empty()
			},
			_ => panic!("ASTORE: Expected ReturnAddress/Reference"),
		}
	}

	fn length(&self) -> U2 {
		2
	}

	fn to_string(&self) -> String {
		format!("astore {}", self.index)
	}
}