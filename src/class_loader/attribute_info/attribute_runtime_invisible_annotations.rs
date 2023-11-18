use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::{attribute_runtime_visibile_annotations::Annotation, Attribute};

#[derive(Debug, Clone)]
pub struct AttributeRuntimeInvisibleAnnotations {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
}

impl Attribute for AttributeRuntimeInvisibleAnnotations {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeRuntimeInvisibleAnnotations {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let num_annotations = parser.consume_u2();
        let mut annotations = Vec::with_capacity(num_annotations as usize);
        for _ in 0..num_annotations {
            annotations.push(Annotation::new(parser));
        }

        AttributeRuntimeInvisibleAnnotations {
            attribute_name_index,
            attribute_length,
            num_annotations,
            annotations,
        }
    }
}
