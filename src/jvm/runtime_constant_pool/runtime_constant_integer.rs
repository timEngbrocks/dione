use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantInteger {
	pub value: i32,
}

impl RuntimeConstant for RuntimeConstantInteger {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let value = resolve_constant!(ConstantPoolInfoType::Integer, index, constant_pool).to_i32();

		RuntimeConstantInteger {
			value,
		}
    }
}