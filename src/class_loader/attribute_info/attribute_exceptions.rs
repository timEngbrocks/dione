use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeExceptions {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_exceptions: U2,
    exception_index_table: Vec<U2>,
}

impl Attribute for AttributeExceptions {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeExceptions {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let number_of_exceptions = parser.consume_u2();
        let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
        for _ in 0..number_of_exceptions {
            exception_index_table.push(parser.consume_u2());
        }

        AttributeExceptions {
            attribute_name_index,
            attribute_length,
            number_of_exceptions,
            exception_index_table,
        }
    }
}
