use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeNestHost {
    attribute_name_index: U2,
    attribute_length: U4,
    host_class_index: U2,
}

impl Attribute for AttributeNestHost {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeNestHost {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let host_class_index = parser.consume_u2();

        AttributeNestHost {
            attribute_name_index,
            attribute_length,
            host_class_index,
        }
    }
}
