use crate::{class_loader::{constant_pool_info::ConstantPoolInfoType, class_file::ClassFile}, resolve_constant};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct SymRefClassOrInterface {
	pub name: String
}

impl RuntimeConstant for SymRefClassOrInterface {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
		let class = resolve_constant!(ConstantPoolInfoType::Class, index, class_file.constant_pool);
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, class.name_index, class_file.constant_pool).to_string();

		SymRefClassOrInterface {
			name
		}
    }
}

// FIXME: impl Loadable for SymRefClassOrInterface