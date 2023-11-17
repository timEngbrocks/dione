use crate::{
    class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType},
    jvm::{object_manager::ObjectManager, types::reference::Reference},
    resolve_constant,
};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct StringConstant {
    text: String,
}

impl StringConstant {
    pub fn get(&self) -> Reference {
        ObjectManager::get_string_instance(self.text.clone())
    }

    pub fn text(&self) -> &String {
        &self.text
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
