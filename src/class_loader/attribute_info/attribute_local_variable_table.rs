use crate::class_loader::{parser::{Parser, U4, U2}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeLocalVariableTable {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub local_variable_table_length: U2,
	pub local_variable_table: Vec<LocalVariableTableEntry>,
}

impl Attribute for AttributeLocalVariableTable {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeLocalVariableTable {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let local_variable_table_length = parser.consume_u2();
		let mut local_variable_table = Vec::with_capacity(local_variable_table_length as usize);
		for _ in 0..local_variable_table_length {
			local_variable_table.push(LocalVariableTableEntry::new(parser));
		}

		AttributeLocalVariableTable {
			attribute_name_index,
			attribute_length,
			local_variable_table_length,
			local_variable_table,
		}
	}
}

#[derive(Debug, Clone)]
pub struct LocalVariableTableEntry {
	pub start_pc: U2,
	pub length: U2,
	pub name_index: U2,
	pub descriptor_index: U2,
	pub index: U2,
}

impl LocalVariableTableEntry {
	pub fn new(parser: &mut Parser) -> LocalVariableTableEntry {
		let start_pc = parser.consume_u2();
		let length = parser.consume_u2();
		let name_index = parser.consume_u2();
		let descriptor_index = parser.consume_u2();
		let index = parser.consume_u2();

		LocalVariableTableEntry {
			start_pc,
			length,
			name_index,
			descriptor_index,
			index,
		}
	}
}