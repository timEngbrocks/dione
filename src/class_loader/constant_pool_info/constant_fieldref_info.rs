use crate::class_loader::parser::Parser;

use super::{ConstantPoolInfo, super::parser::{U1, U2}};

#[derive(Clone)]
pub struct ConstantFieldrefInfo {
	pub tag: U1,
	pub class_index: U2,
	pub name_and_type_index: U2,
}

impl ConstantPoolInfo for ConstantFieldrefInfo {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.consume_u1();
		let class_index = parser.consume_u2();
		let name_and_type_index = parser.consume_u2();
		ConstantFieldrefInfo { tag, class_index, name_and_type_index }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}