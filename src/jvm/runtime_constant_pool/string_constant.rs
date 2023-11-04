use crate::{
    class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType},
    jvm::types::reference::Reference,
    resolve_constant,
};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct StringConstant {
    text: String,
}

impl StringConstant {
    pub fn get(&self) -> Reference {
        unimplemented!("StringConstant::get")
    }
}

impl RuntimeConstant for StringConstant {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
        let string = resolve_constant!(
            ConstantPoolInfoType::String,
            index,
            class_file.constant_pool
        );
        let text = resolve_constant!(
            ConstantPoolInfoType::Utf8,
            string.string_index,
            class_file.constant_pool
        )
        .to_string();

        StringConstant { text }
    }
}

// FIXME: impl Loadable for StringConstant
