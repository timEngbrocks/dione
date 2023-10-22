use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantClass {
	pub name: String,
}

impl RuntimeConstant for RuntimeConstantClass {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let class = resolve_constant!(ConstantPoolInfoType::Class, index, constant_pool);
		let name = resolve_constant!(ConstantPoolInfoType::Utf8, class.name_index, constant_pool).to_string();

		RuntimeConstantClass {
			name,
		}
    }
}