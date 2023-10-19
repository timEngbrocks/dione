use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U4}};

pub struct ConstantLongInfo {
	pub tag: U1,
	pub high_bytes: U4,
	pub low_bytes: U4,
}

impl ConstantPoolInfo for ConstantLongInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let high_bytes = parser.consume_u4();
		let low_bytes = parser.consume_u4();
		ConstantLongInfo { tag, high_bytes, low_bytes }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}