use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeSignature {
    attribute_name_index: U2,
    attribute_length: U4,
    signature_index: U2,
}

impl Attribute for AttributeSignature {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeSignature {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let signature_index = parser.consume_u2();

        AttributeSignature {
            attribute_name_index,
            attribute_length,
            signature_index,
        }
    }
}
