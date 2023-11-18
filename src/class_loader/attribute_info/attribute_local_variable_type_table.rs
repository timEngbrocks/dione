use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeLocalVariableTypeTable {
    attribute_name_index: U2,
    attribute_length: U4,
    local_variable_type_table_length: U2,
    local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

impl Attribute for AttributeLocalVariableTypeTable {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeLocalVariableTypeTable {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let local_variable_type_table_length = parser.consume_u2();
        let mut local_variable_type_table =
            Vec::with_capacity(local_variable_type_table_length as usize);
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

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTableEntry {
    start_pc: U2,
    length: U2,
    name_index: U2,
    signature_index: U2,
    index: U2,
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
