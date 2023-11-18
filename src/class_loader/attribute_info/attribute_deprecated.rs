use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeDeprecated {
    attribute_name_index: U2,
    attribute_length: U4,
}

impl Attribute for AttributeDeprecated {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeDeprecated {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();

        AttributeDeprecated {
            attribute_name_index,
            attribute_length,
        }
    }
}
