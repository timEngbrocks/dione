use crate::class_loader::{parser::{Parser, U2, U4}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug)]
pub struct AttributeLineNumberTable {
	pub attribute_name_index: U2,
	pub attribute_length: U4,
	pub line_number_table_length: U2,
	pub line_number_table: Vec<LineNumberTableEntry>,
}

impl Attribute for AttributeLineNumberTable {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeLineNumberTable {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let line_number_table_length = parser.consume_u2();
		let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
		for _ in 0..line_number_table_length {
			line_number_table.push(LineNumberTableEntry::new(parser));
		}

		AttributeLineNumberTable {
			attribute_name_index,
			attribute_length,
			line_number_table_length,
			line_number_table,
		}
	}
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
	pub start_pc: U2,
	pub line_number: U2,
}

impl LineNumberTableEntry {
	pub fn new(parser: &mut Parser) -> LineNumberTableEntry {
		let start_pc = parser.consume_u2();
		let line_number = parser.consume_u2();

		LineNumberTableEntry {
			start_pc,
			line_number
		}
	}
}