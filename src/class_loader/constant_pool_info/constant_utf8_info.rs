use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U2}};

pub struct ConstantUtf8Info {
	pub tag: U1,
	pub length: U2,
	pub bytes: Vec<U1>,
}

impl ConstantPoolInfo for ConstantUtf8Info {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let length = parser.consume_u2();
		let mut bytes = Vec::with_capacity(length as usize);
		for _ in 0..length {
			bytes.push(parser.consume_u1());
		}
		ConstantUtf8Info { tag, length, bytes }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}