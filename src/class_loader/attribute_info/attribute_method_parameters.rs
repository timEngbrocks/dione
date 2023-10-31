use crate::class_loader::{parser::{U4, U2, Parser, U1}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeMethodParameters {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
	pub parameters_count: U1,
	pub parameters: Vec<MethodParameter>,
}

impl Attribute for AttributeMethodParameters {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeMethodParameters {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let parameters_count = parser.consume_u1();
		let mut parameters = Vec::with_capacity(parameters_count as usize);
		for _ in 0..parameters_count {
			parameters.push(MethodParameter::new(parser));
		}

		AttributeMethodParameters {
			attribute_name_index,
			attribute_length,
			parameters_count,
			parameters,
		}
	}
}

#[derive(Debug, Clone)]
pub struct MethodParameter {
	pub name_index: U2,
    pub access_flags: U2,
}

impl MethodParameter {
	fn new(parser: &mut Parser) -> MethodParameter {
		let name_index = parser.consume_u2();
		let access_flags = parser.consume_u2();

		MethodParameter {
			name_index,
			access_flags,
		}
	}
}
