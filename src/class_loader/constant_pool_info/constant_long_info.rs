use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U4},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantLongInfo {
    tag: U1,
    high_bytes: U4,
    low_bytes: U4,
}

impl ConstantLongInfo {
    pub fn to_i64(&self) -> i64 {
        (i64::from_be(self.high_bytes as i64) << 32) + i64::from_be(self.low_bytes as i64)
    }
}

impl ConstantPoolInfo for ConstantLongInfo {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let high_bytes = parser.consume_u4();
        let low_bytes = parser.consume_u4();
        ConstantLongInfo {
            tag,
            high_bytes,
            low_bytes,
        }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
