use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantStringInfo {
    tag: U1,
    string_index: U2,
}
impl ConstantStringInfo {
    pub fn string_index(&self) -> &U2 {
        &self.string_index
    }
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
