use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U1, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeMethodParameters {
    attribute_name_index: U2,
    attribute_length: U4,
    parameters_count: U1,
    parameters: Vec<MethodParameter>,
}

impl Attribute for AttributeMethodParameters {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeMethodParameters {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let parameters_count = parser.consume_u1();
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        for _ in 0..parameters_count {
            parameters.push(MethodParameter::new(parser));
        }

        AttributeMethodParameters {
            attribute_name_index,
            attribute_length,
            parameters_count,
            parameters,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
    name_index: U2,
    access_flags: U2,
}

impl MethodParameter {
    fn new(parser: &mut Parser) -> MethodParameter {
        let name_index = parser.consume_u2();
        let access_flags = parser.consume_u2();

        MethodParameter {
            name_index,
            access_flags,
        }
    }
}
