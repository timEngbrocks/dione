use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{int::Int, Value}}, class_loader::parser::{Parser, U2}, opcodes};

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
		let padding = parser.align(4);
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

	fn to_string(&self) -> String {
		format!("lookupswitch {}, {}, {}, {:?}", self.padding, self.default, self.npairs, self.match_offset_pairs)
	}
}