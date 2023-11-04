use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantInvokeDynamicInfo {
    pub tag: U1,
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

impl ConstantPoolInfo for ConstantInvokeDynamicInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let bootstrap_method_attr_index = parser.consume_u2();
        let name_and_type_index = parser.consume_u2();
        ConstantInvokeDynamicInfo {
            tag,
            bootstrap_method_attr_index,
            name_and_type_index,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
