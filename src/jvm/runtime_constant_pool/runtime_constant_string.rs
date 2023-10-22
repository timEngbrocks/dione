use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantString {
	pub value: String,
}

impl RuntimeConstant for RuntimeConstantString {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let string = resolve_constant!(ConstantPoolInfoType::String, index, constant_pool);
		let value = resolve_constant!(ConstantPoolInfoType::Utf8, string.string_index, constant_pool).to_string();

		RuntimeConstantString {
			value,
		}
    }
}