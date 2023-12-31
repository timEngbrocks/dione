use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantInvokeDynamicInfo {
    tag: U1,
    bootstrap_method_attr_index: U2,
    name_and_type_index: U2,
}
impl ConstantInvokeDynamicInfo {
    pub fn bootstrap_method_attr_index(&self) -> &U2 {
        &self.bootstrap_method_attr_index
    }

    pub fn name_and_type_index(&self) -> &U2 {
        &self.name_and_type_index
    }
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
