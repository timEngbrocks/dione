use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeLocalVariableTypeTable {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub local_variable_type_table_length: U2,
	pub local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

impl Attribute for AttributeLocalVariableTypeTable {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeLocalVariableTypeTable {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let local_variable_type_table_length = parser.consume_u2();
		let mut local_variable_type_table = Vec::with_capacity(local_variable_type_table_length as usize);
		for _ in 0..local_variable_type_table_length {
			local_variable_type_table.push(LocalVariableTypeTableEntry::new(parser));
		}

		AttributeLocalVariableTypeTable {
			attribute_name_index,
			attribute_length,
			local_variable_type_table_length,
			local_variable_type_table,
		}
	}
}

#[derive(Debug)]
pub struct LocalVariableTypeTableEntry {
	pub start_pc: U2,
	pub length: U2,
	pub name_index: U2,
	pub signature_index: U2,
	pub index: U2,
}

impl LocalVariableTypeTableEntry {
	fn new(parser: &mut Parser) -> LocalVariableTypeTableEntry {
		let start_pc = parser.consume_u2();
		let length = parser.consume_u2();
		let name_index = parser.consume_u2();
		let signature_index = parser.consume_u2();
		let index = parser.consume_u2();

		LocalVariableTypeTableEntry {
			start_pc,
			length,
			name_index,
			signature_index,
			index,
		}
	}
}
