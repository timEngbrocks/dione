use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeSignature {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub signature_index: U2,
}

impl Attribute for AttributeSignature {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeSignature {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let signature_index = parser.consume_u2();

		AttributeSignature {
			attribute_name_index,
			attribute_length,
			signature_index,
		}
	}
}
