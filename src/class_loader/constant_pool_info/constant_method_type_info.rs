use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U2}};

pub struct ConstantMethodTypeInfo {
	pub tag: U1,
	pub descriptor_index: U2,
}

impl ConstantPoolInfo for ConstantMethodTypeInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let descriptor_index = parser.consume_u2();
		ConstantMethodTypeInfo { tag, descriptor_index }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}