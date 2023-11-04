use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult, BranchKind}, types::{Types, Value}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPEQ {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPEQ {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPEQ));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPEQ {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 == value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmpeq: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPNE {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPNE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPNE));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPNE {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 != value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmpne: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPLT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPLT));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPLT {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 < value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmplt: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGE {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPGE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPGE));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPGE {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 >= value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmpge: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGT {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPGT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPGT));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPGT {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 > value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmpgt: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLE {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IF_ICMPLE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPLE));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IF_ICMPLE {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
		let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
		match execution_context.stack.pop() {
			Types::Int(value2) => {
				match execution_context.stack.pop() {
					Types::Int(value1) => {
						let value2 = value2.get();
						let value1 = value1.get();
						if value1 <= value2 {
							InstructionResult::branch(BranchKind::Relative(offset))
						} else {
							InstructionResult::empty()
						}
					},
					_ => panic!("Expected Int"),
				}
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}

	fn to_string(&self) -> String {
		format!("if_icmple: {}, {}", self.branchbyte1, self.branchbyte2)
	}
}