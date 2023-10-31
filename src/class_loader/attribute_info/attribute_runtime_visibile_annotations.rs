use core::panic;

use crate::class_loader::{parser::{U4, U2, U1, Parser}, constant_pool_info::ConstantPool};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeRuntimeVisibleAnnotations {
	pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub num_annotations: U2,
    pub annotations: Vec<Annotation>,
}

impl Attribute for AttributeRuntimeVisibleAnnotations {
	fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeRuntimeVisibleAnnotations {
		let attribute_name_index = parser.consume_u2();
		let attribute_length = parser.consume_u4();
		let num_annotations = parser.consume_u2();
		let mut annotations = Vec::with_capacity(num_annotations as usize);
		for _ in 0..num_annotations {
			annotations.push(Annotation::new(parser));
		}

		AttributeRuntimeVisibleAnnotations {
			attribute_name_index,
			attribute_length,
			num_annotations,
			annotations,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Annotation {
	pub type_index: U2,
	pub num_element_value_pairs: U2,
	pub element_value_pairs: Vec<ElementValuePair>,
} 

impl Annotation {
	pub fn new(parser: &mut Parser) -> Annotation {
		let type_index = parser.consume_u2();
		let num_element_value_pairs = parser.consume_u2();
		let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
		for _ in 0..num_element_value_pairs {
			element_value_pairs.push(ElementValuePair::new(parser));
		}

		Annotation {
			type_index,
			num_element_value_pairs,
			element_value_pairs,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ElementValuePair {
	pub element_name_index: U2,
	pub value: ElementValue,
} 

impl ElementValuePair {
	pub fn new(parser: &mut Parser) -> ElementValuePair {
		let element_name_index = parser.consume_u2();
		let value = ElementValue::new(parser);

		ElementValuePair {
			element_name_index,
			value,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ElementValue {
	pub tag: U1,
	pub value: Value
}

impl ElementValue {
	pub fn new(parser: &mut Parser) -> ElementValue {
		let tag = parser.consume_u1();
		let value = Value::new(tag, parser);

		ElementValue {
			tag,
			value,
		}
	}
}

#[derive(Debug, Clone)]
pub enum Value {
	Constant(ValueConstant),
	Enum(ValueEnum),
	Class(ValueClass),
	Annotation(Annotation),
	Array(ValueArray),
}

impl Value {
	pub fn new(tag: U1, parser: &mut Parser) -> Value {
		match tag {
			b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
				Value::Constant(ValueConstant::new(parser))
			},
			b'e' => {
				Value::Enum(ValueEnum::new(parser))
			},
			b'c' => {
				Value::Class(ValueClass::new(parser))
			},
			b'@' => {
				Value::Annotation(Annotation::new(parser))
			},
			b'[' => {
				Value::Array(ValueArray::new(parser))
			},
			v => panic!("Unknown tag: {v}")
		}
	}
}

#[derive(Debug, Clone)]
pub struct ValueConstant {
	pub const_value_index: U2,
}

impl ValueConstant {
	pub fn new(parser: &mut Parser) -> ValueConstant {
		let const_value_index = parser.consume_u2();

		ValueConstant {
			const_value_index,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ValueEnum {
	pub type_name_index: U2,
	pub const_name_index: U2,
}

impl ValueEnum {
	pub fn new(parser: &mut Parser) -> ValueEnum {
		let type_name_index = parser.consume_u2();
		let const_name_index = parser.consume_u2();

		ValueEnum {
			type_name_index,
			const_name_index,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ValueClass {
	pub class_info_index: U2,
}

impl ValueClass {
	pub fn new(parser: &mut Parser) -> ValueClass {
		let class_info_index = parser.consume_u2();

		ValueClass {
			class_info_index,
		}
	}
}

#[derive(Debug, Clone)]
pub struct ValueArray {
	pub num_values: U2,
	pub values: Vec<ElementValue>,
}

impl ValueArray {
	pub fn new(parser: &mut Parser) -> ValueArray {
		let num_values = parser.consume_u2();
		let mut values = Vec::with_capacity(num_values as usize);
		for _ in 0..num_values {
			values.push(ElementValue::new(parser));
		}

		ValueArray {
			num_values,
			values,
		}
	}
}
