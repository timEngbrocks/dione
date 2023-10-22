use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U4}};

pub struct ConstantIntegerInfo {
	pub tag: U1,
	pub bytes: U4,
}

impl ConstantIntegerInfo {
	pub fn to_i32(&self) -> i32 {
		i32::from_be(self.bytes as i32)
	}
}

impl ConstantPoolInfo for ConstantIntegerInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let bytes = parser.consume_u4();
		ConstantIntegerInfo { tag, bytes }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}