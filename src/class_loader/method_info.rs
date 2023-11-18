use super::{
    attribute_info::{Attribute, AttributeInfo},
    constant_pool_info::ConstantPool,
    parser::{Parser, U2},
};

#[derive(Debug)]
pub enum MethodAccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

pub type MethodAccessFlag = u16;

#[derive(Debug, Clone)]
pub struct MethodInfo {
    access_flags: MethodAccessFlag,
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> MethodInfo {
        let access_flags = parser.consume_u2();
        let name_index = parser.consume_u2();
        let descriptor_index = parser.consume_u2();
        let attributes_count = parser.consume_u2();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(parser, constant_pool));
        }

        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }

    pub fn access_flags(&self) -> &MethodAccessFlag {
        &self.access_flags
    }

    pub fn name_index(&self) -> &U2 {
        &self.name_index
    }

    pub fn descriptor_index(&self) -> &U2 {
        &self.descriptor_index
    }

    pub fn attributes_count(&self) -> &U2 {
        &self.attributes_count
    }

    pub fn attributes(&self) -> &Vec<AttributeInfo> {
        &self.attributes
    }
}
