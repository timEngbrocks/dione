use crate::{jvm::{frame::Frame, instructions::Instruction, runtime_constant_pool::{RuntimeConstants, runtime_constant_integer::RuntimeConstantInteger, runtime_constant_float::RuntimeConstantFloat, runtime_constant_string::RuntimeConstantString, runtime_constant_class::RuntimeConstantClass, runtime_constant_long::RuntimeConstantLong, runtime_constant_double::RuntimeConstantDouble}, types::{int::Int, Types, Value, float::Float, double::Double, long::Long}}, class_loader::parser::{Parser, U2, U1}, opcodes};

#[allow(non_camel_case_types)]
pub struct LDC {
	index: U1,
}
impl Instruction for LDC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LDC));
		let index = parser.consume_u1();
		LDC {
			index,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		ldc_impl(execution_context, self.index as u16);
	}

	fn length(&self) -> U2 {
		2
	}
}
#[allow(non_camel_case_types)]
pub struct LDC_W {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for LDC_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LDC_W));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		LDC_W {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) {
		let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
		ldc_impl(execution_context, index);
	}

	fn length(&self) -> U2 {
		3
	}
}
#[allow(non_camel_case_types)]
pub struct LDC2_W {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for LDC2_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LDC2_W));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		LDC2_W {
			indexbyte1,
			indexbyte2,
		}
	}

	// TODO: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc2_w
	fn execute(&mut self, execution_context: &mut Frame) {
		let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
		match execution_context.constant_pool.get(index) {
			RuntimeConstants::Long(RuntimeConstantLong { value }) => {
				execution_context.stack.push(Types::Long(Long::from_value(*value)));
			},
			RuntimeConstants::Double(RuntimeConstantDouble { value }) => {
				execution_context.stack.push(Types::Double(Double::from_value(*value)));
			},
			// TODO: Dynamically computed long/double constant
			_ => {
				panic!("LDC2_W::execute: Unknown constant at index {}", index);
			},
		}
	}

	fn length(&self) -> U2 {
		3
	}
}

// TODO: https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html#jvms-6.5.ldc
fn ldc_impl(execution_context: &mut Frame, index: u16) {
	match execution_context.constant_pool.get(index) {
		RuntimeConstants::Integer(RuntimeConstantInteger { value }) => {
			execution_context.stack.push(Types::Int(Int::from_value(*value)));
		},
		RuntimeConstants::Float(RuntimeConstantFloat { value }) => {
			execution_context.stack.push(Types::Float(Float::from_value(*value)));
		},
		RuntimeConstants::String(RuntimeConstantString { .. }) => {
			// TODO: Push reference to an instance of class String that contains value
			// TODO: Push reference to the above instance
			unimplemented!("LDC::execute: String");
		},
		RuntimeConstants::Class(RuntimeConstantClass { .. }) => {
			// TODO: Resolve named Class/Interface and push reference to it
			unimplemented!("LDC::execute: Class");
		},
		// TODO: method type, method handle, dynamically computed constant
		_ => {
			panic!("LDC::execute: Unknown constant at index {}", index);
		},
	}
}