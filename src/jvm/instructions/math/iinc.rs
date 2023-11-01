use crate::{jvm::{frame::Frame, types::{int::Int, Value, Types}, instructions::{Instruction, InstructionResult}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IINC {
	index: U1,
	constant: U1,
}
impl Instruction for IINC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IINC));
		let index = parser.consume_u1();
		let constant = parser.consume_u1();
		IINC {
			index,
			constant,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let constant = self.constant as i32;
		match execution_context.local_variables.get(self.index as u16).clone() {
			Types::Int(value) => {
				execution_context.local_variables.set(self.index as u16, Types::Int(Int::from_value(value.get().clone() + constant)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int"),
		}
	}

	fn length(&self) -> U2 {
		3
	}
}