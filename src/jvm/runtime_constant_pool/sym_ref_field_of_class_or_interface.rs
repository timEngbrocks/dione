use crate::{
    class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType},
    resolve_constant,
};

use super::{sym_ref_class_or_interface::SymRefClassOrInterface, RuntimeConstant};

#[derive(Clone)]
pub struct SymRefFieldOfClassOrInterface {
    name: String,
    descriptor: String,
    class_ref: SymRefClassOrInterface,
}
impl SymRefFieldOfClassOrInterface {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn descriptor(&self) -> &String {
        &self.descriptor
    }

    pub fn class_ref(&self) -> &SymRefClassOrInterface {
        &self.class_ref
    }
}
impl RuntimeConstant for SymRefFieldOfClassOrInterface {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
        let field = resolve_constant!(
            ConstantPoolInfoType::Fieldref,
            &index,
            class_file.constant_pool()
        );
        let name_and_type = resolve_constant!(
            ConstantPoolInfoType::NameAndType,
            field.name_and_type_index(),
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
        let class_ref = SymRefClassOrInterface::resolve(*field.class_index(), class_file);

        SymRefFieldOfClassOrInterface {
            name,
            descriptor,
            class_ref,
        }
    }
}
