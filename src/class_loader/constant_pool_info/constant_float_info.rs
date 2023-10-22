use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U4}};

pub struct ConstantFloatInfo {
	pub tag: U1,
	pub bytes: U4,
}

impl ConstantFloatInfo {
	pub fn to_f32(&self) -> f32 {
		self.bytes as f32
	}
}

impl ConstantPoolInfo for ConstantFloatInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let bytes = parser.consume_u4();
		ConstantFloatInfo { tag, bytes }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}