use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeConstantValue {
    attribute_name_index: U2,
    attribute_length: U4,
    constantvalue_index: U2,
}
impl AttributeConstantValue {
    pub fn placeholder(length: U4) -> AttributeConstantValue {
        AttributeConstantValue {
            attribute_name_index: 0,
            attribute_length: length,
            constantvalue_index: 0,
        }
    }
}
impl Attribute for AttributeConstantValue {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeConstantValue {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        assert_eq!(attribute_length, 2);
        let constantvalue_index = parser.consume_u2();

        AttributeConstantValue {
            attribute_name_index,
            attribute_length,
            constantvalue_index,
        }
    }
}
