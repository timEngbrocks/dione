use core::panic;

use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U1, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeRuntimeVisibleAnnotations {
    attribute_name_index: U2,
    attribute_length: U4,
    num_annotations: U2,
    annotations: Vec<Annotation>,
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
    type_index: U2,
    num_element_value_pairs: U2,
    element_value_pairs: Vec<ElementValuePair>,
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
    element_name_index: U2,
    value: ElementValue,
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
    tag: U1,
    value: Value,
}

impl ElementValue {
    pub fn new(parser: &mut Parser) -> ElementValue {
        let tag = parser.consume_u1();
        let value = Value::new(tag, parser);

        ElementValue { tag, value }
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
    fn new(tag: U1, parser: &mut Parser) -> Value {
        match tag {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
                Value::Constant(ValueConstant::new(parser))
            }
            b'e' => Value::Enum(ValueEnum::new(parser)),
            b'c' => Value::Class(ValueClass::new(parser)),
            b'@' => Value::Annotation(Annotation::new(parser)),
            b'[' => Value::Array(ValueArray::new(parser)),
            v => panic!("Unknown tag: {v}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueConstant {
    const_value_index: U2,
}

impl ValueConstant {
    pub fn new(parser: &mut Parser) -> ValueConstant {
        let const_value_index = parser.consume_u2();

        ValueConstant { const_value_index }
    }
}

#[derive(Debug, Clone)]
pub struct ValueEnum {
    type_name_index: U2,
    const_name_index: U2,
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
    class_info_index: U2,
}

impl ValueClass {
    pub fn new(parser: &mut Parser) -> ValueClass {
        let class_info_index = parser.consume_u2();

        ValueClass { class_info_index }
    }
}

#[derive(Debug, Clone)]
pub struct ValueArray {
    num_values: U2,
    values: Vec<ElementValue>,
}

impl ValueArray {
    pub fn new(parser: &mut Parser) -> ValueArray {
        let num_values = parser.consume_u2();
        let mut values = Vec::with_capacity(num_values as usize);
        for _ in 0..num_values {
            values.push(ElementValue::new(parser));
        }

        ValueArray { num_values, values }
    }
}
