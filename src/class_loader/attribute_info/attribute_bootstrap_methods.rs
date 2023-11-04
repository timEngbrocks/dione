use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeBootstrapMethods {
    pub attribute_name_index: U2,
    pub attribute_length: U4,
    pub num_bootstrap_methods: U2,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

impl Attribute for AttributeBootstrapMethods {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeBootstrapMethods {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let num_bootstrap_methods = parser.consume_u2();
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethod::new(parser));
        }

        AttributeBootstrapMethods {
            attribute_name_index,
            attribute_length,
            num_bootstrap_methods,
            bootstrap_methods,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: U2,
    pub num_bootstrap_arguments: U2,
    pub bootstrap_arguments: Vec<U2>,
}

impl BootstrapMethod {
    pub fn new(parser: &mut Parser) -> BootstrapMethod {
        let bootstrap_method_ref = parser.consume_u2();
        let num_bootstrap_arguments = parser.consume_u2();
        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(parser.consume_u2());
        }

        BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        }
    }
}
