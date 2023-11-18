use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantNameAndTypeInfo {
    tag: U1,
    name_index: U2,
    descriptor_index: U2,
}
impl ConstantNameAndTypeInfo {
    pub fn name_index(&self) -> &U2 {
        &self.name_index
    }

    pub fn descriptor_index(&self) -> &U2 {
        &self.descriptor_index
    }
}
impl ConstantPoolInfo for ConstantNameAndTypeInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let name_index = parser.consume_u2();
        let descriptor_index = parser.consume_u2();
        ConstantNameAndTypeInfo {
            tag,
            name_index,
            descriptor_index,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
