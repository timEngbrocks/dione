use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeDeprecated {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
}

impl Attribute for AttributeDeprecated {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeDeprecated {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();

		AttributeDeprecated {
			attribute_name_index,
			attribute_length,
		}
	}
}
