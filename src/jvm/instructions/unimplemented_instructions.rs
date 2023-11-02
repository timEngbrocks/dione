use crate::{jvm::{frame::Frame, types::{int::Int, Value}}, class_loader::parser::{Parser, U2, U1}, opcodes};

use super::{Instruction, InstructionResult};

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
pub struct TABLESWITCH {
	padding: usize,
	default: i32,
	low: i32,
	high: i32,
	jump_offsets: Vec<i32>,
}
impl Instruction for TABLESWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::TABLESWITCH));
		let padding = 4 - (parser.offset() % 4);
		for _ in 0..padding {
			parser.consume_u1();
		}
		let defaultbyte1 = parser.consume_u1();
		let defaultbyte2 = parser.consume_u1();
		let defaultbyte3 = parser.consume_u1();
		let defaultbyte4 = parser.consume_u1();
		let default = (defaultbyte1 as i32) << 24 | (defaultbyte2 as i32) << 16 | (defaultbyte3 as i32) << 8 | defaultbyte4 as i32;
		let lowbyte1 = parser.consume_u1();
		let lowbyte2 = parser.consume_u1();
		let lowbyte3 = parser.consume_u1();
		let lowbyte4 = parser.consume_u1();
		let low = (lowbyte1 as i32) << 24 | (lowbyte2 as i32) << 16 | (lowbyte3 as i32) << 8 | lowbyte4 as i32;
		let highbyte1 = parser.consume_u1();
		let highbyte2 = parser.consume_u1();
		let highbyte3 = parser.consume_u1();
		let highbyte4 = parser.consume_u1();
		let high = (highbyte1 as i32) << 24 | (highbyte2 as i32) << 16 | (highbyte3 as i32) << 8 | highbyte4 as i32;
		assert!(high >= low);
		let num_jump_offsets = high - low + 1;
		let mut jump_offsets = Vec::with_capacity(num_jump_offsets as usize);
		for _ in 0..num_jump_offsets {
			let offsetbyte1 = parser.consume_u1();
			let offsetbyte2 = parser.consume_u1();
			let offsetbyte3 = parser.consume_u1();
			let offsetbyte4 = parser.consume_u1();
			let offset = (offsetbyte1 as i32) << 24 | (offsetbyte2 as i32) << 16 | (offsetbyte3 as i32) << 8 | offsetbyte4 as i32;
			jump_offsets.push(offset);
		}
		TABLESWITCH {
			padding,
			default,
			low,
			high,
			jump_offsets,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		(1 + self.padding as u32 + 12 + 4 * (self.high - self.low + 1) as u32) as u16
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LOOKUPSWITCH {
	padding: usize,
	default: i32,
	npairs: i32,
	match_offset_pairs: Vec<(Int, i32)>,
}
impl Instruction for LOOKUPSWITCH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::LOOKUPSWITCH));
		let padding = 4 - (parser.offset() % 4);
		for _ in 0..padding {
			parser.consume_u1();
		}
		let defaultbyte1 = parser.consume_u1();
		let defaultbyte2 = parser.consume_u1();
		let defaultbyte3 = parser.consume_u1();
		let defaultbyte4 = parser.consume_u1();
		let default = (defaultbyte1 as i32) << 24 | (defaultbyte2 as i32) << 16 | (defaultbyte3 as i32) << 8 | defaultbyte4 as i32;
		let npairs1 = parser.consume_u1();
		let npairs2 = parser.consume_u1();
		let npairs3 = parser.consume_u1();
		let npairs4 = parser.consume_u1();
		let npairs = (npairs1 as i32) << 24 | (npairs2 as i32) << 16 | (npairs3 as i32) << 8 | npairs4 as i32;
		assert!(npairs >= 0);
		let mut match_offset_pairs = Vec::with_capacity(npairs as usize);
		for _ in 0..npairs {
			let matchbyte1 = parser.consume_u1();
			let matchbyte2 = parser.consume_u1();
			let matchbyte3 = parser.consume_u1();
			let matchbyte4 = parser.consume_u1();
			let match_value = (matchbyte1 as i32) << 24 | (matchbyte2 as i32) << 16 | (matchbyte3 as i32) << 8 | matchbyte4 as i32;
			let offsetbyte1 = parser.consume_u1();
			let offsetbyte2 = parser.consume_u1();
			let offsetbyte3 = parser.consume_u1();
			let offsetbyte4 = parser.consume_u1();
			let offset = (offsetbyte1 as i32) << 24 | (offsetbyte2 as i32) << 16 | (offsetbyte3 as i32) << 8 | offsetbyte4 as i32;
			match_offset_pairs.push((Int::from_value(match_value), offset));
		}
		LOOKUPSWITCH {
			padding,
			default,
			npairs,
			match_offset_pairs,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		(1 + self.padding as u32 + 8 + 8 * std::cmp::max(self.npairs, 0) as u32) as u16
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
pub struct IFNULL {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IFNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IFNULL));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IFNULL {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFNONNULL {
	branchbyte1: U1,
	branchbyte2: U1,
}
impl Instruction for IFNONNULL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::IFNONNULL));
		let branchbyte1 = parser.consume_u1();
		let branchbyte2 = parser.consume_u1();
		IFNONNULL {
			branchbyte1,
			branchbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
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
pub struct GETFIELD {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for GETFIELD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::GETFIELD));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		GETFIELD {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PUTFIELD {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for PUTFIELD {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::PUTFIELD));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		PUTFIELD {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEINTERFACE {
	indexbyte1: U1,
	indexbyte2: U1,
	count: U1,
}
impl Instruction for INVOKEINTERFACE {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKEINTERFACE));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		let count = parser.consume_u1();
		let byte4 = parser.consume_u1();
		assert_eq!(byte4, 0);
		INVOKEINTERFACE {
			indexbyte1,
			indexbyte2,
			count,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		5
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEDYNAMIC {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for INVOKEDYNAMIC {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKEDYNAMIC));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		let byte3 = parser.consume_u1();
		assert_eq!(byte3, 0);
		let byte4 = parser.consume_u1();
		assert_eq!(byte4, 0);
		INVOKEDYNAMIC {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		5
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARRAYLENGTH {}
impl Instruction for ARRAYLENGTH {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ARRAYLENGTH));
		ARRAYLENGTH {}
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
pub struct CHECKCAST {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for CHECKCAST {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::CHECKCAST));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		CHECKCAST {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INSTANCEOF {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for INSTANCEOF {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INSTANCEOF));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		INSTANCEOF {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		3
	}
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MONITORENTER {}
impl Instruction for MONITORENTER {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::MONITORENTER));
		MONITORENTER {}
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
		MONITOREXIT {}
	}

	fn execute(&mut self, _: &mut Frame) -> InstructionResult {
		unimplemented!()
	}

	fn length(&self) -> U2 {
		1
	}
}