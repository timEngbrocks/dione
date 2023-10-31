use crate::class_loader::{parser::{U2, U4, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributePermittedSubclasses {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub number_of_classes: U2,
	pub classes: Vec<U2>,
}

impl Attribute for AttributePermittedSubclasses {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributePermittedSubclasses {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let number_of_classes = parser.consume_u2();
		let mut classes = Vec::with_capacity(number_of_classes as usize);
		for _ in 0..number_of_classes {
			classes.push(parser.consume_u2());
		}

		AttributePermittedSubclasses {
			attribute_name_index,
			attribute_length,
			number_of_classes,
			classes,
		}
	}
}