use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::{Attribute, AttributeInfo};

#[derive(Debug, Clone)]
pub struct AttributeRecord {
    attribute_name_index: U2,
    attribute_length: U4,
    components_count: U2,
    components: Vec<RecordComponentInfo>,
}

impl Attribute for AttributeRecord {
    fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> AttributeRecord {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let components_count = parser.consume_u2();
        let mut components = Vec::with_capacity(components_count as usize);
        for _ in 0..components_count {
            components.push(RecordComponentInfo::new(parser, constant_pool));
        }

        AttributeRecord {
            attribute_name_index,
            attribute_length,
            components_count,
            components,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordComponentInfo {
    name_index: U2,
    descriptor_index: U2,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl RecordComponentInfo {
    fn new(parser: &mut Parser, constant_pool: &ConstantPool) -> RecordComponentInfo {
        let name_index = parser.consume_u2();
        let descriptor_index = parser.consume_u2();
        let attributes_count = parser.consume_u2();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(parser, constant_pool));
        }

        RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}
