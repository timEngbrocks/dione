use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::{Attribute, AttributeInfo};

#[derive(Debug)]
pub struct AttributeRecord {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
	pub components_count: U2,
	pub components: Vec<RecordComponentInfo>,
}

impl Attribute for AttributeRecord {
	fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> AttributeRecord {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let components_count = parser.consume_u2();
		let mut components = Vec::with_capacity(components_count as usize);
		for _ in 0..components_count {
			components.push(RecordComponentInfo::new(parser, constant_pool));
		}

		AttributeRecord {
			attribute_name_index,
			attribute_length,
			components_count,
			components,
		}
	}
}

#[derive(Debug)]
pub struct RecordComponentInfo {
	pub name_index: U2,
	pub descriptor_index: U2,
	pub attributes_count: U2,
	pub attributes: Vec<AttributeInfo>,
}

impl RecordComponentInfo {
	fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> RecordComponentInfo {
		let name_index = parser.consume_u2();
		let descriptor_index = parser.consume_u2();
		let attributes_count = parser.consume_u2();
		let mut attributes = Vec::with_capacity(attributes_count as usize);
		for _ in 0..attributes_count {
			attributes.push(AttributeInfo::new(parser, constant_pool));
		}

		RecordComponentInfo {
			name_index,
			descriptor_index,
			attributes_count,
			attributes,
		}
	}
}