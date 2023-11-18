use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::{attribute_runtime_visibile_annotations::ElementValue, Attribute};

#[derive(Debug, Clone)]
pub struct AttributeAnnotationDefault {
    attribute_name_index: U2,
    attribute_length: U4,
    default_value: ElementValue,
}

impl Attribute for AttributeAnnotationDefault {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeAnnotationDefault {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let default_value = ElementValue::new(parser);

        AttributeAnnotationDefault {
            attribute_name_index,
            attribute_length,
            default_value,
        }
    }
}
