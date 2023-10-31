use crate::{resolve_constant, class_loader::{constant_pool_info::ConstantPoolInfoType, class_file::ClassFile}};

use super::{RuntimeConstant, sym_ref_class_or_interface::SymRefClassOrInterface};

#[derive(Clone)]
pub struct SymRefFieldOfClassOrInterface {
	pub name: String,
	pub descriptor: String,
	pub class_ref: SymRefClassOrInterface,
}

impl RuntimeConstant for SymRefFieldOfClassOrInterface {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
		let field = resolve_constant!(ConstantPoolInfoType::Fieldref, index, class_file.constant_pool);
		let name_and_type = resolve_constant!(ConstantPoolInfoType::NameAndType, field.name_and_type_index, class_file.constant_pool);
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, name_and_type.name_index, class_file.constant_pool).to_string();
		let descriptor = resolve_constant!(ConstantPoolInfoType::Utf8, name_and_type.descriptor_index, class_file.constant_pool).to_string();
		let class_ref = SymRefClassOrInterface::resolve(field.class_index, class_file);

		SymRefFieldOfClassOrInterface {
			name,
			descriptor,
			class_ref,
		}
    }
}