use crate::{
    class_loader::{
        attribute_info::AttributeInfo, class_file::ClassFile,
        constant_pool_info::ConstantPoolInfoType,
    },
    resolve_constant,
};

use super::{
    sym_ref_method_handle::SymRefMethodHandle, RuntimeConstant, RuntimeConstantPool,
    RuntimeConstants,
};

#[derive(Clone)]
pub struct SymRefDynamicallyComputedCallSite {
    method_handle_ref: SymRefMethodHandle,
    arguments: Vec<RuntimeConstants>,
    name: String,
    descriptor: String,
}

impl RuntimeConstant for SymRefDynamicallyComputedCallSite {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
        let invokedynamic = resolve_constant!(
            ConstantPoolInfoType::InvokeDynamic,
            &index,
            class_file.constant_pool()
        );

        let bootstrap_method_attr =
            match class_file.attributes().iter().reduce(|attr, e| match attr {
                AttributeInfo::BootstrapMethods(_) => attr,
                _ => e,
            }) {
                Some(AttributeInfo::BootstrapMethods(bootstrap_method)) => bootstrap_method,
                _ => panic!("No BootstrapMethods attribute found in class file"),
            };

        let bootstrap_method = bootstrap_method_attr
            .bootstrap_methods()
            .get(*invokedynamic.bootstrap_method_attr_index() as usize)
            .unwrap();
        let method_handle_ref =
            SymRefMethodHandle::resolve(*bootstrap_method.bootstrap_method_ref(), class_file);
        let arguments = bootstrap_method
            .bootstrap_arguments()
            .iter()
            .map(|x| match RuntimeConstantPool::resolve(*x, class_file) {
                Some(constant) => constant,
                None => panic!("No constant found at index {}", x),
            })
            .collect::<Vec<RuntimeConstants>>();
        let name_and_type = resolve_constant!(
            ConstantPoolInfoType::NameAndType,
            invokedynamic.name_and_type_index(),
            class_file.constant_pool()
        );
        let name = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            name_and_type.name_index(),
            class_file.constant_pool()
        )
        .to_string();
        let descriptor = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            name_and_type.descriptor_index(),
            class_file.constant_pool()
        )
        .to_string();

        SymRefDynamicallyComputedCallSite {
            method_handle_ref,
            arguments,
            name,
            descriptor,
        }
    }
}

// FIXME: impl Loadable for SymRefDynamicallyComputedCallSite
