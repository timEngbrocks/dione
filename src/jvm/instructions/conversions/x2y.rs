use crate::{jvm::{frame::Frame, types::{Value, float::Float, double::Double, int::Int}, instructions::{Instruction, InstructionResult}, types::{Types, long::Long}}, class_loader::parser::{Parser, U2}, opcodes};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2L {}
impl Instruction for I2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2L));
		I2L {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				// Note: Rust's as cast uses sign extension
				execution_context.stack.push(Types::Long(Long::from_value(value.get() as i64)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2F {}
impl Instruction for I2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2F));
		I2F {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				// NOTE: Possible FIXME: as I am not sure that Rust's as uses roundTiesToEven for rounding.
				execution_context.stack.push(Types::Float(Float::from_value(value.get() as f32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2D {}
impl Instruction for I2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2D));
		I2D {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				execution_context.stack.push(Types::Long(Long::from_value(value.get() as i64)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct L2I {}
impl Instruction for L2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::L2I));
		L2I {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value) => {
				execution_context.stack.push(Types::Int(Int::from_value((value.get() & 0xFFFFFFFF) as i32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Long on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct L2F {}
impl Instruction for L2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::L2F));
		L2F {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value) => {
				// NOTE: Possible FIXME: as I am not sure that Rust's as uses roundTiesToEven for rounding.
				execution_context.stack.push(Types::Float(Float::from_value(value.get() as f32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Long on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct L2D {}
impl Instruction for L2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::L2D));
		L2D {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Long(value) => {
				// NOTE: Possible FIXME: as I am not sure that Rust's as uses roundTiesToEven for rounding.
				execution_context.stack.push(Types::Double(Double::from_value(value.get() as f64)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Long on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct F2I {}
impl Instruction for F2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::F2I));
		F2I {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value) => {
				let x = value.get();
				if x.is_nan() {
					execution_context.stack.push(Types::Int(Int::from_value(0)));
				} else if !x.is_infinite() {
					execution_context.stack.push(Types::Int(Int::from_value(x.trunc() as i32)));
				} else if (x.is_finite() && x >= i32::MAX as f32) || (x.is_infinite() && x > 0.0) {
					execution_context.stack.push(Types::Int(Int::from_value(i32::MAX)));
				} else if (x.is_finite() && x <= -i32::MAX as f32) || (x.is_infinite() && x < 0.0) {
					execution_context.stack.push(Types::Int(Int::from_value(-i32::MAX)));
				} else {
					panic!("Unexpected float value, don't know how to convert to int")
				}
				InstructionResult::empty()
			},
			_ => panic!("Expected Float on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct F2L {}
impl Instruction for F2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::F2L));
		F2L {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value) => {
				let x = value.get();
				if x.is_nan() {
					execution_context.stack.push(Types::Long(Long::from_value(0)));
				} else if !x.is_infinite() {
					execution_context.stack.push(Types::Long(Long::from_value(x.trunc() as i64)));
				} else if (x.is_finite() && x >= i64::MAX as f32) || (x.is_infinite() && x > 0.0) {
					execution_context.stack.push(Types::Long(Long::from_value(i64::MAX)));
				} else if (x.is_finite() && x <= -i64::MAX as f32) || (x.is_infinite() && x < 0.0) {
					execution_context.stack.push(Types::Long(Long::from_value(-i64::MAX)));
				} else {
					panic!("Unexpected float value, don't know how to convert to long")
				}
				InstructionResult::empty()
			},
			_ => panic!("Expected Float on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct F2D {}
impl Instruction for F2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::F2D));
		F2D {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Float(value) => {
				execution_context.stack.push(Types::Double(Double::from_value(value.get() as f64)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Float on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D2I {}
impl Instruction for D2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::D2I));
		D2I {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value) => {
				let x = value.get();
				if x.is_nan() {
					execution_context.stack.push(Types::Int(Int::from_value(0)));
				} else if !x.is_infinite() {
					execution_context.stack.push(Types::Int(Int::from_value(x.trunc() as i32)));
				} else if (x.is_finite() && x >= i32::MAX as f64) || (x.is_infinite() && x > 0.0) {
					execution_context.stack.push(Types::Int(Int::from_value(i32::MAX)));
				} else if (x.is_finite() && x <= -i32::MAX as f64) || (x.is_infinite() && x < 0.0) {
					execution_context.stack.push(Types::Int(Int::from_value(-i32::MAX)));
				} else {
					panic!("Unexpected double value, don't know how to convert to int")
				}
				InstructionResult::empty()
			},
			_ => panic!("Expected Double on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D2L {}
impl Instruction for D2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::D2L));
		D2L {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value) => {
				let x = value.get();
				if x.is_nan() {
					execution_context.stack.push(Types::Long(Long::from_value(0)));
				} else if !x.is_infinite() {
					execution_context.stack.push(Types::Long(Long::from_value(x.trunc() as i64)));
				} else if (x.is_finite() && x >= i64::MAX as f64) || (x.is_infinite() && x > 0.0) {
					execution_context.stack.push(Types::Long(Long::from_value(i64::MAX)));
				} else if (x.is_finite() && x <= -i64::MAX as f64) || (x.is_infinite() && x < 0.0) {
					execution_context.stack.push(Types::Long(Long::from_value(-i64::MAX)));
				} else {
					panic!("Unexpected double value, don't know how to convert to long")
				}
				InstructionResult::empty()
			},
			_ => panic!("Expected Double on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D2F {}
impl Instruction for D2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::D2F));
		D2F {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Double(value) => {
				let x = value.get();
				if x.is_nan() {
					execution_context.stack.push(Types::Float(Float::from_value(f32::NAN)));
				} else if x.is_finite() && x.abs() < f32::MIN as f64 {
					execution_context.stack.push(Types::Float(Float::from_value(x.signum() as f32 * 0.0)));
				} else if x.is_finite() && x.abs() > f32::MAX as f64 {
					execution_context.stack.push(Types::Float(Float::from_value(x.signum() as f32 * f32::INFINITY)));
				} else {
					execution_context.stack.push(Types::Float(Float::from_value(value.get() as f32)));
				}
				InstructionResult::empty()
			},
			_ => panic!("Expected Double on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2B {}
impl Instruction for I2B {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2B));
		I2B {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				execution_context.stack.push(Types::Int(Int::from_value(((value.get() & 0xFF) as i8) as i32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2C {}
impl Instruction for I2C {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2C));
		I2C {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				// NOTE: Possible FIXME: as I am not sure that Rust's as uses zero extension in this case.
				execution_context.stack.push(Types::Int(Int::from_value(((value.get() & 0xFFFF) as i16) as i32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2S {}
impl Instruction for I2S {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2S));
		I2S {}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		match execution_context.stack.pop() {
			Types::Int(value) => {
				// NOTE: Possible FIXME: as I am not sure that Rust's as uses zero extension in this case.
				execution_context.stack.push(Types::Int(Int::from_value(((value.get() & 0xFFFF) as i16) as i32)));
				InstructionResult::empty()
			},
			_ => panic!("Expected Int on top of stack")
		}
	}

	fn length(&self) -> U2 {
		1
	}
}