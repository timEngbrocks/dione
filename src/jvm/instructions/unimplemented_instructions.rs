use crate::{jvm::{instructions::Instructions, execution_context::ExecutionContext}, class_loader::parser::{Parser, U2}};

use super::Instruction;

#[allow(non_camel_case_types)]
pub struct ACONST_NULL {}
impl Instruction for ACONST_NULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ACONST_NULL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_M1 {}
impl Instruction for ICONST_M1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_M1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_0 {}
impl Instruction for ICONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_1 {}
impl Instruction for ICONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_2 {}
impl Instruction for ICONST_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_3 {}
impl Instruction for ICONST_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_4 {}
impl Instruction for ICONST_4 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_4 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ICONST_5 {}
impl Instruction for ICONST_5 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ICONST_5 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LCONST_0 {}
impl Instruction for LCONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LCONST_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LCONST_1 {}
impl Instruction for LCONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LCONST_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FCONST_0 {}
impl Instruction for FCONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FCONST_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FCONST_1 {}
impl Instruction for FCONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FCONST_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FCONST_2 {}
impl Instruction for FCONST_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FCONST_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DCONST_0 {}
impl Instruction for DCONST_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DCONST_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DCONST_1 {}
impl Instruction for DCONST_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DCONST_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct BIPUSH {}
impl Instruction for BIPUSH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::BIPUSH as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct SIPUSH {}
impl Instruction for SIPUSH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::SIPUSH as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LDC {}
impl Instruction for LDC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LDC as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LDC_W {}
impl Instruction for LDC_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LDC_W as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LDC2_W {}
impl Instruction for LDC2_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LDC2_W as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ILOAD {}
impl Instruction for ILOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ILOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LLOAD {}
impl Instruction for LLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LLOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FLOAD {}
impl Instruction for FLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FLOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DLOAD {}
impl Instruction for DLOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DLOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD {}
impl Instruction for ALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ILOAD_0 {}
impl Instruction for ILOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ILOAD_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ILOAD_1 {}
impl Instruction for ILOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ILOAD_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ILOAD_2 {}
impl Instruction for ILOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ILOAD_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ILOAD_3 {}
impl Instruction for ILOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ILOAD_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LLOAD_0 {}
impl Instruction for LLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LLOAD_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LLOAD_1 {}
impl Instruction for LLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LLOAD_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LLOAD_2 {}
impl Instruction for LLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LLOAD_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LLOAD_3 {}
impl Instruction for LLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LLOAD_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FLOAD_0 {}
impl Instruction for FLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FLOAD_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FLOAD_1 {}
impl Instruction for FLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FLOAD_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FLOAD_2 {}
impl Instruction for FLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FLOAD_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FLOAD_3 {}
impl Instruction for FLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FLOAD_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DLOAD_0 {}
impl Instruction for DLOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DLOAD_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DLOAD_1 {}
impl Instruction for DLOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DLOAD_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DLOAD_2 {}
impl Instruction for DLOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DLOAD_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DLOAD_3 {}
impl Instruction for DLOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DLOAD_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_0 {}
impl Instruction for ALOAD_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ALOAD_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_1 {}
impl Instruction for ALOAD_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ALOAD_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_2 {}
impl Instruction for ALOAD_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ALOAD_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ALOAD_3 {}
impl Instruction for ALOAD_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ALOAD_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IALOAD {}
impl Instruction for IALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LALOAD {}
impl Instruction for LALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FALOAD {}
impl Instruction for FALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DALOAD {}
impl Instruction for DALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct AALOAD {}
impl Instruction for AALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::AALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct BALOAD {}
impl Instruction for BALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::BALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct CALOAD {}
impl Instruction for CALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::CALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct SALOAD {}
impl Instruction for SALOAD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::SALOAD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISTORE {}
impl Instruction for ISTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSTORE {}
impl Instruction for LSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSTORE {}
impl Instruction for FSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSTORE {}
impl Instruction for DSTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ASTORE {}
impl Instruction for ASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISTORE_0 {}
impl Instruction for ISTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISTORE_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISTORE_1 {}
impl Instruction for ISTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISTORE_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISTORE_2 {}
impl Instruction for ISTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISTORE_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISTORE_3 {}
impl Instruction for ISTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISTORE_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSTORE_0 {}
impl Instruction for LSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSTORE_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSTORE_1 {}
impl Instruction for LSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSTORE_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSTORE_2 {}
impl Instruction for LSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSTORE_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSTORE_3 {}
impl Instruction for LSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSTORE_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSTORE_0 {}
impl Instruction for FSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSTORE_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSTORE_1 {}
impl Instruction for FSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSTORE_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSTORE_2 {}
impl Instruction for FSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSTORE_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSTORE_3 {}
impl Instruction for FSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSTORE_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSTORE_0 {}
impl Instruction for DSTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSTORE_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSTORE_1 {}
impl Instruction for DSTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSTORE_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSTORE_2 {}
impl Instruction for DSTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSTORE_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSTORE_3 {}
impl Instruction for DSTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSTORE_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ASTORE_0 {}
impl Instruction for ASTORE_0 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ASTORE_0 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ASTORE_1 {}
impl Instruction for ASTORE_1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ASTORE_1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ASTORE_2 {}
impl Instruction for ASTORE_2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ASTORE_2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ASTORE_3 {}
impl Instruction for ASTORE_3 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ASTORE_3 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IASTORE {}
impl Instruction for IASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LASTORE {}
impl Instruction for LASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FASTORE {}
impl Instruction for FASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DASTORE {}
impl Instruction for DASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct AASTORE {}
impl Instruction for AASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::AASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct BASTORE {}
impl Instruction for BASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::BASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct CASTORE {}
impl Instruction for CASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::CASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct SASTORE {}
impl Instruction for SASTORE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::SASTORE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct POP {}
impl Instruction for POP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::POP as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct POP2 {}
impl Instruction for POP2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::POP2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP {}
impl Instruction for DUP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP_X1 {}
impl Instruction for DUP_X1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP_X1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP_X2 {}
impl Instruction for DUP_X2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP_X2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP2 {}
impl Instruction for DUP2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP2_X1 {}
impl Instruction for DUP2_X1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP2_X1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DUP2_X2 {}
impl Instruction for DUP2_X2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DUP2_X2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct SWAP {}
impl Instruction for SWAP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::SWAP as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IADD {}
impl Instruction for IADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IADD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LADD {}
impl Instruction for LADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LADD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FADD {}
impl Instruction for FADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FADD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DADD {}
impl Instruction for DADD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DADD as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISUB {}
impl Instruction for ISUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISUB as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSUB {}
impl Instruction for LSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSUB as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FSUB {}
impl Instruction for FSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FSUB as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DSUB {}
impl Instruction for DSUB {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DSUB as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IMUL {}
impl Instruction for IMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IMUL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LMUL {}
impl Instruction for LMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LMUL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FMUL {}
impl Instruction for FMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FMUL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DMUL {}
impl Instruction for DMUL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DMUL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IDIV {}
impl Instruction for IDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IDIV as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LDIV {}
impl Instruction for LDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LDIV as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FDIV {}
impl Instruction for FDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FDIV as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DDIV {}
impl Instruction for DDIV {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DDIV as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IREM {}
impl Instruction for IREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IREM as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LREM {}
impl Instruction for LREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LREM as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FREM {}
impl Instruction for FREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FREM as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DREM {}
impl Instruction for DREM {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DREM as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct INEG {}
impl Instruction for INEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::INEG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LNEG {}
impl Instruction for LNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LNEG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FNEG {}
impl Instruction for FNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FNEG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DNEG {}
impl Instruction for DNEG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DNEG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISHL {}
impl Instruction for ISHL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISHL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSHL {}
impl Instruction for LSHL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSHL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ISHR {}
impl Instruction for ISHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ISHR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LSHR {}
impl Instruction for LSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LSHR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IUSHR {}
impl Instruction for IUSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IUSHR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LUSHR {}
impl Instruction for LUSHR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LUSHR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IAND {}
impl Instruction for IAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IAND as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LAND {}
impl Instruction for LAND {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LAND as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IOR {}
impl Instruction for IOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IOR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LOR {}
impl Instruction for LOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LOR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IXOR {}
impl Instruction for IXOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IXOR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LXOR {}
impl Instruction for LXOR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LXOR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IINC {}
impl Instruction for IINC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IINC as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2L {}
impl Instruction for I2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2L as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2F {}
impl Instruction for I2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2F as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2D {}
impl Instruction for I2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2D as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct L2I {}
impl Instruction for L2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::L2I as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct L2F {}
impl Instruction for L2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::L2F as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct L2D {}
impl Instruction for L2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::L2D as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct F2I {}
impl Instruction for F2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::F2I as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct F2L {}
impl Instruction for F2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::F2L as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct F2D {}
impl Instruction for F2D {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::F2D as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct D2I {}
impl Instruction for D2I {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::D2I as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct D2L {}
impl Instruction for D2L {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::D2L as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct D2F {}
impl Instruction for D2F {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::D2F as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2B {}
impl Instruction for I2B {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2B as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2C {}
impl Instruction for I2C {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2C as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct I2S {}
impl Instruction for I2S {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::I2S as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LCMP {}
impl Instruction for LCMP {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LCMP as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FCMPL {}
impl Instruction for FCMPL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FCMPL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FCMPG {}
impl Instruction for FCMPG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FCMPG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DCMPL {}
impl Instruction for DCMPL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DCMPL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DCMPG {}
impl Instruction for DCMPG {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DCMPG as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFEQ {}
impl Instruction for IFEQ {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFEQ as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFNE {}
impl Instruction for IFNE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFNE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFLT {}
impl Instruction for IFLT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFLT as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFGE {}
impl Instruction for IFGE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFGE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFGT {}
impl Instruction for IFGT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFGT as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFLE {}
impl Instruction for IFLE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFLE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPEQ {}
impl Instruction for IF_ICMPEQ {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPEQ as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPNE {}
impl Instruction for IF_ICMPNE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPNE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPLT {}
impl Instruction for IF_ICMPLT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPLT as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPGE {}
impl Instruction for IF_ICMPGE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPGE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPGT {}
impl Instruction for IF_ICMPGT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPGT as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ICMPLE {}
impl Instruction for IF_ICMPLE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ICMPLE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ACMPEQ {}
impl Instruction for IF_ACMPEQ {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ACMPEQ as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IF_ACMPNE {}
impl Instruction for IF_ACMPNE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IF_ACMPNE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct GOTO {}
impl Instruction for GOTO {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::GOTO as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct JSR {}
impl Instruction for JSR {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::JSR as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct RET {}
impl Instruction for RET {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::RET as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct TABLESWITCH {}
impl Instruction for TABLESWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::TABLESWITCH as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LOOKUPSWITCH {}
impl Instruction for LOOKUPSWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LOOKUPSWITCH as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IRETURN {}
impl Instruction for IRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IRETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct LRETURN {}
impl Instruction for LRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::LRETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct FRETURN {}
impl Instruction for FRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::FRETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct DRETURN {}
impl Instruction for DRETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::DRETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct ARETURN {}
impl Instruction for ARETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::ARETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct RETURN {}
impl Instruction for RETURN {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::RETURN as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct WIDE {}
impl Instruction for WIDE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::WIDE as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct MULTIANEWARRAY {}
impl Instruction for MULTIANEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::MULTIANEWARRAY as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFNULL {}
impl Instruction for IFNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFNULL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IFNONNULL {}
impl Instruction for IFNONNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IFNONNULL as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct GOTO_W {}
impl Instruction for GOTO_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::GOTO_W as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct JSR_W {}
impl Instruction for JSR_W {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::JSR_W as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct BREAKPOINT {}
impl Instruction for BREAKPOINT {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::BREAKPOINT as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IMPDEP1 {}
impl Instruction for IMPDEP1 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IMPDEP1 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}
#[allow(non_camel_case_types)]
pub struct IMPDEP2 {}
impl Instruction for IMPDEP2 {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert!(opcode == (Instructions::IMPDEP2 as u8));
		todo!("Implement");
	}

	fn execute(&mut self, _: &ExecutionContext) {}

	fn length(&self) -> U2 {
		1
	}
}