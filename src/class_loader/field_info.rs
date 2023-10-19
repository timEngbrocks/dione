use super::{attribute_info::{AttributeInfo, Attribute}, parser::Parser, constant_pool_info::ConstantPool};

#[derive(Debug)]
pub enum FieldAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Volatile = 0x0040,
    Transient = 0x0080,
    Synthetic = 0x1000,
    Enum = 0x4000,
}

pub type FieldAccessFlag = u16;

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: FieldAccessFlag,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
	pub fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> FieldInfo {
		let access_flags = parser.consume_u2();
		let name_index = parser.consume_u2();
		let descriptor_index = parser.consume_u2();
		let attributes_count = parser.consume_u2();
		let mut attributes = Vec::with_capacity(attributes_count as usize);
		for _ in 0..attributes_count {
			attributes.push(AttributeInfo::new(parser, constant_pool));
		}

		FieldInfo {
			access_flags,
			name_index,
			descriptor_index,
			attributes_count,
			attributes
		}
	}
}