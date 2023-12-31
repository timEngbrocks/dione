use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeLineNumberTable {
    attribute_name_index: U2,
    attribute_length: U4,
    line_number_table_length: U2,
    line_number_table: Vec<LineNumberTableEntry>,
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

#[derive(Debug, Clone)]
pub struct LineNumberTableEntry {
    start_pc: U2,
    line_number: U2,
}

impl LineNumberTableEntry {
    pub fn new(parser: &mut Parser) -> LineNumberTableEntry {
        let start_pc = parser.consume_u2();
        let line_number = parser.consume_u2();

        LineNumberTableEntry {
            start_pc,
            line_number,
        }
    }
}
