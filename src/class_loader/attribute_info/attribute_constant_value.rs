use crate::class_loader::{parser::{Parser, U2, U4}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeConstantValue {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub constantvalue_index: U2,
}

impl Attribute for AttributeConstantValue {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeConstantValue {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		assert_eq!(attribute_length, 2);
		let constantvalue_index = parser.consume_u2();

		AttributeConstantValue {
			attribute_name_index,
			attribute_length,
			constantvalue_index
		}
	}
}