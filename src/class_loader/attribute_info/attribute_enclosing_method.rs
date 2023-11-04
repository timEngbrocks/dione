use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeEnclosingMethod {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub class_index: U2,
    pub method_index: U2,
}

impl Attribute for AttributeEnclosingMethod {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeEnclosingMethod {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let class_index = parser.consume_u2();
        let method_index = parser.consume_u2();

        AttributeEnclosingMethod {
            attribute_name_index,
            attribute_length,
            class_index,
            method_index,
        }
    }
}
