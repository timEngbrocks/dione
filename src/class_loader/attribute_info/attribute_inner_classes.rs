use crate::class_loader::{parser::{Parser, U2, U4}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeInnerClasses {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub number_of_classes: U2,
	pub classes: Vec<InnerClassesListEntry>,
}

impl Attribute for AttributeInnerClasses {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeInnerClasses {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let number_of_classes = parser.consume_u2();
		let mut classes = Vec::with_capacity(number_of_classes as usize);
		for _ in 0..number_of_classes {
			classes.push(InnerClassesListEntry::new(parser));
		}

		AttributeInnerClasses {
			attribute_name_index,
			attribute_length,
			number_of_classes,
			classes,
		}
	}
}

#[derive(Debug)]
pub struct InnerClassesListEntry {
	pub inner_class_info_index: U2,
	pub outer_class_info_index: U2,
	pub inner_name_index: U2,
	pub inner_class_access_flags: U2
}

impl InnerClassesListEntry {
	pub fn new(parser: &mut Parser) -> InnerClassesListEntry {
		let inner_class_info_index = parser.consume_u2();
		let outer_class_info_index = parser.consume_u2();
		let inner_name_index = parser.consume_u2();
		let inner_class_access_flags = parser.consume_u2();

		InnerClassesListEntry {
			inner_class_info_index,
			outer_class_info_index,
			inner_name_index,
			inner_class_access_flags,
		}
	}
}