use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantPackageInfo {
    pub tag: U1,
    pub name_index: U2,
}

impl ConstantPoolInfo for ConstantPackageInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let name_index = parser.consume_u2();
        ConstantPackageInfo { tag, name_index }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
