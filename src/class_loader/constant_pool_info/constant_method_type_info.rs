use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantMethodTypeInfo {
    tag: U1,
    descriptor_index: U2,
}
impl ConstantMethodTypeInfo {
    pub fn descriptor_index(&self) -> &U2 {
        &self.descriptor_index
    }
}
impl ConstantPoolInfo for ConstantMethodTypeInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let descriptor_index = parser.consume_u2();
        ConstantMethodTypeInfo {
            tag,
            descriptor_index,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
