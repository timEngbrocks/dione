use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeNestMembers {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub number_of_classes: U2,
    pub classes: Vec<U2>,
}

impl Attribute for AttributeNestMembers {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeNestMembers {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let number_of_classes = parser.consume_u2();
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        for _ in 0..number_of_classes {
            classes.push(parser.consume_u2());
        }

        AttributeNestMembers {
            attribute_name_index,
            attribute_length,
            number_of_classes,
            classes,
        }
    }
}
