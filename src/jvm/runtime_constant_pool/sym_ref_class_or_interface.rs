use crate::{class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfoType}, resolve_constant};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct SymRefClassOrInterface {
	pub name: String
}

impl RuntimeConstant for SymRefClassOrInterface {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let class = resolve_constant!(ConstantPoolInfoType::Class, index, constant_pool);
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, class.name_index, constant_pool).to_string();

		SymRefClassOrInterface {
			name
		}
    }
}

// FIXME: impl Loadable for SymRefClassOrInterface