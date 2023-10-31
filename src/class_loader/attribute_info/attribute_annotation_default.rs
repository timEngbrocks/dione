use crate::class_loader::{parser::{U4, U2, Parser}, constant_pool_info::ConstantPool};

use super::{Attribute, attribute_runtime_visibile_annotations::ElementValue};

#[derive(Debug, Clone)]
pub struct AttributeAnnotationDefault {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
	pub default_value: ElementValue
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
