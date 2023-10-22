use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U4}};

pub struct ConstantDoubleInfo {
	pub tag: U1,
	pub high_bytes: U4,
	pub low_bytes: U4,
}

impl ConstantDoubleInfo {
	pub fn to_f64(&self) -> f64 {
		(((self.high_bytes as i64) << 32) + (self.low_bytes as i64)) as f64
	}
}

impl ConstantPoolInfo for ConstantDoubleInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let high_bytes = parser.consume_u4();
		let low_bytes = parser.consume_u4();
		ConstantDoubleInfo { tag, high_bytes, low_bytes }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}