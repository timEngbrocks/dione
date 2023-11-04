use crate::class_loader::parser::Parser;

use super::{
    super::parser::{U1, U2},
    ConstantPoolInfo,
};

#[derive(Clone)]
pub struct ConstantUtf8Info {
    pub tag: U1,
    pub length: U2,
    pub bytes: Vec<U1>,
}

#[allow(clippy::inherent_to_string)]
impl ConstantUtf8Info {
    pub fn to_string(&self) -> String {
        match String::from_utf8(self.bytes.clone()) {
            Ok(v) => v,
            Err(_) => panic!("Could not create string from bytes!"),
        }
    }
}

impl ConstantPoolInfo for ConstantUtf8Info {
    fn new(parser: &mut Parser) -> Self {
        let tag = parser.consume_u1();
        let length = parser.consume_u2();
        let mut bytes = Vec::with_capacity(length as usize);
        for _ in 0..length {
            bytes.push(parser.consume_u1());
        }
        ConstantUtf8Info { tag, length, bytes }
    }

    fn get_tag(&self) -> &U1 {
        &self.tag
    }
}
