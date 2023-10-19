use crate::class_loader::parser::{Parser, U2};

use super::{ConstantPoolInfo, super::parser::U1};

pub struct ConstantMethodHandleInfo {
	pub tag: U1,
	pub reference_kind: U1,
	pub reference_index: U2,
}

impl ConstantPoolInfo for ConstantMethodHandleInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let reference_kind = parser.consume_u1();
		let reference_index = parser.consume_u2();
		ConstantMethodHandleInfo { tag, reference_kind, reference_index }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}