use crate::{class_loader::{constant_pool_info::ConstantPoolInfoType, class_file::ClassFile}, resolve_constant};

use super::{RuntimeConstant, sym_ref_class_or_interface::SymRefClassOrInterface};

#[derive(Clone)]
pub struct SymRefMethodOfClass {
	pub name: String,
	pub descriptor: String,
	pub class_ref: SymRefClassOrInterface,
}

impl RuntimeConstant for SymRefMethodOfClass {
	fn resolve(index: u16, class_file: &ClassFile) -> Self {
		let method = resolve_constant!(ConstantPoolInfoType::Methodref, index, class_file.constant_pool);
		let name_and_type = resolve_constant!(ConstantPoolInfoType::NameAndType, method.name_and_type_index, class_file.constant_pool);
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, name_and_type.name_index, class_file.constant_pool).to_string();
		let descriptor = resolve_constant!(ConstantPoolInfoType::Utf8, name_and_type.descriptor_index, class_file.constant_pool).to_string();
		let class_ref = SymRefClassOrInterface::resolve(method.class_index, class_file);

		SymRefMethodOfClass {
			name,
			descriptor,
			class_ref,
		}
	}
}