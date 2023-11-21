use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantDynamicInfo {
    tag: U1,
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}

impl ConstantPoolInfo for ConstantDynamicInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let bootstrap_method_attr_index = parser.consume_u2();
        let name_and_type_index = parser.consume_u2();
        ConstantDynamicInfo {
            tag,
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
