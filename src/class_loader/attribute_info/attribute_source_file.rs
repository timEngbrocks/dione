use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeSourceFile {
    attribute_name_index: U2,
    attribute_length: U4,
    source_file_index: U2,
}

impl Attribute for AttributeSourceFile {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeSourceFile {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let source_file_index = parser.consume_u2();

        AttributeSourceFile {
            attribute_name_index,
            attribute_length,
            source_file_index,
        }
    }
}
