use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantStringInfo {
    pub tag: U1,
    pub string_index: U2,
}

impl ConstantPoolInfo for ConstantStringInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let string_index = parser.consume_u2();
        ConstantStringInfo { tag, string_index }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
