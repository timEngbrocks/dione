use crate::{jvm::frame::Frame, class_loader::parser::{Parser, U2}, opcodes};

use super::{Instruction, InstructionResult};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IADD {}
impl Instruction for IADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IADD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LADD {}
impl Instruction for LADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LADD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FADD {}
impl Instruction for FADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FADD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DADD {}
impl Instruction for DADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DADD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISUB {}
impl Instruction for ISUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ISUB));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSUB {}
impl Instruction for LSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSUB));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSUB {}
impl Instruction for FSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FSUB));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSUB {}
impl Instruction for DSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DSUB));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMUL {}
impl Instruction for IMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IMUL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LMUL {}
impl Instruction for LMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LMUL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FMUL {}
impl Instruction for FMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FMUL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DMUL {}
impl Instruction for DMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DMUL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IDIV {}
impl Instruction for IDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IDIV));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LDIV {}
impl Instruction for LDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LDIV));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FDIV {}
impl Instruction for FDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FDIV));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DDIV {}
impl Instruction for DDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DDIV));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IREM {}
impl Instruction for IREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IREM));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LREM {}
impl Instruction for LREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LREM));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FREM {}
impl Instruction for FREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FREM));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DREM {}
impl Instruction for DREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DREM));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INEG {}
impl Instruction for INEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INEG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LNEG {}
impl Instruction for LNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LNEG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FNEG {}
impl Instruction for FNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FNEG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DNEG {}
impl Instruction for DNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DNEG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISHL {}
impl Instruction for ISHL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ISHL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSHL {}
impl Instruction for LSHL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSHL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISHR {}
impl Instruction for ISHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ISHR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSHR {}
impl Instruction for LSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LSHR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IUSHR {}
impl Instruction for IUSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IUSHR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LUSHR {}
impl Instruction for LUSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LUSHR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IAND {}
impl Instruction for IAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IAND));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LAND {}
impl Instruction for LAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LAND));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IOR {}
impl Instruction for IOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IOR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LOR {}
impl Instruction for LOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LOR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IXOR {}
impl Instruction for IXOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IXOR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LXOR {}
impl Instruction for LXOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LXOR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IINC {}
impl Instruction for IINC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IINC));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct I2L {}
impl Instruction for I2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::I2L));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
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
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LCMP {}
impl Instruction for LCMP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LCMP));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FCMPL {}
impl Instruction for FCMPL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FCMPL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FCMPG {}
impl Instruction for FCMPG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::FCMPG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DCMPL {}
impl Instruction for DCMPL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DCMPL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DCMPG {}
impl Instruction for DCMPG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::DCMPG));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPEQ {}
impl Instruction for IF_ICMPEQ {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPEQ));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPNE {}
impl Instruction for IF_ICMPNE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPNE));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {}
impl Instruction for IF_ICMPLT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPLT));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGE {}
impl Instruction for IF_ICMPGE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPGE));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPGT {}
impl Instruction for IF_ICMPGT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPGT));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ICMPLE {}
impl Instruction for IF_ICMPLE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IF_ICMPLE));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JSR {}
impl Instruction for JSR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::JSR));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RET {}
impl Instruction for RET {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::RET));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct TABLESWITCH {}
impl Instruction for TABLESWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::TABLESWITCH));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LOOKUPSWITCH {}
impl Instruction for LOOKUPSWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LOOKUPSWITCH));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WIDE {}
impl Instruction for WIDE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::WIDE));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MULTIANEWARRAY {}
impl Instruction for MULTIANEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::MULTIANEWARRAY));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFNULL {}
impl Instruction for IFNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IFNULL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFNONNULL {}
impl Instruction for IFNONNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IFNONNULL));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GOTO_W {}
impl Instruction for GOTO_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::GOTO_W));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JSR_W {}
impl Instruction for JSR_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::JSR_W));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BREAKPOINT {}
impl Instruction for BREAKPOINT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::BREAKPOINT));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMPDEP1 {}
impl Instruction for IMPDEP1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IMPDEP1));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMPDEP2 {}
impl Instruction for IMPDEP2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IMPDEP2));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GETSTATIC {}
impl Instruction for GETSTATIC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::GETSTATIC));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PUTSTATIC {}
impl Instruction for PUTSTATIC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::PUTSTATIC));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GETFIELD {}
impl Instruction for GETFIELD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::GETFIELD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PUTFIELD {}
impl Instruction for PUTFIELD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::PUTFIELD));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEINTERFACE {}
impl Instruction for INVOKEINTERFACE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKEINTERFACE));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEDYNAMIC {}
impl Instruction for INVOKEDYNAMIC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKEDYNAMIC));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NEWARRAY {}
impl Instruction for NEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::NEWARRAY));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ANEWARRAY {}
impl Instruction for ANEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ANEWARRAY));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARRAYLENGTH {}
impl Instruction for ARRAYLENGTH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ARRAYLENGTH));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ATHROW {}
impl Instruction for ATHROW {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ATHROW));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CHECKCAST {}
impl Instruction for CHECKCAST {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::CHECKCAST));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INSTANCEOF {}
impl Instruction for INSTANCEOF {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INSTANCEOF));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MONITORENTER {}
impl Instruction for MONITORENTER {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::MONITORENTER));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MONITOREXIT {}
impl Instruction for MONITOREXIT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::MONITOREXIT));
		todo!("Implement");
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}