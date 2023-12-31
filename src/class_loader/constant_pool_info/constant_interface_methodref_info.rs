use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantInterfaceMethodrefInfo {
    tag: U1,
    class_index: U2,
    name_and_type_index: U2,
}
impl ConstantInterfaceMethodrefInfo {
    pub fn class_index(&self) -> &U2 {
        &self.class_index
    }

    pub fn name_and_type_index(&self) -> &U2 {
        &self.name_and_type_index
    }
}
impl ConstantPoolInfo for ConstantInterfaceMethodrefInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let class_index = parser.consume_u2();
        let name_and_type_index = parser.consume_u2();
        ConstantInterfaceMethodrefInfo {
            tag,
            class_index,
            name_and_type_index,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
