use crate::{
    class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType},
    resolve_constant,
};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct SymRefMethodType {
    descriptor: String,
}

impl RuntimeConstant for SymRefMethodType {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
        let method_type = resolve_constant!(
            ConstantPoolInfoType::MethodType,
            &index,
            class_file.constant_pool()
        );
        let descriptor = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            method_type.descriptor_index(),
            class_file.constant_pool()
        )
        .to_string();

        SymRefMethodType { descriptor }
    }
}

// FIXME: impl Loadable for SymRefMethodType
