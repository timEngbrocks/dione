use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::{Attribute, attribute_runtime_visibile_annotations::Annotation};

#[derive(Debug)]
pub struct AttributeRuntimeInvisibleAnnotations {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
	pub num_annotations: U2,
	pub annotations: Vec<Annotation>,
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
