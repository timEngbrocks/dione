use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantLong {
	pub value: i64,
}

impl RuntimeConstant for RuntimeConstantLong {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let value = resolve_constant!(ConstantPoolInfoType::Long, index, constant_pool).to_i64();

		RuntimeConstantLong {
			value,
		}
    }
}