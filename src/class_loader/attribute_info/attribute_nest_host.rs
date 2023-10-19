use crate::class_loader::{parser::{U2, U4, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeNestHost {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub host_class_index: U2,
}

impl Attribute for AttributeNestHost {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeNestHost {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let host_class_index = parser.consume_u2();

		AttributeNestHost {
			attribute_name_index,
			attribute_length,
			host_class_index
		}
	}
}