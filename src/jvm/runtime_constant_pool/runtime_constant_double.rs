use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantDouble {
	pub value: f64,
}

impl RuntimeConstant for RuntimeConstantDouble {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let value = resolve_constant!(ConstantPoolInfoType::Double, index, constant_pool).to_f64();

		RuntimeConstantDouble {
			value,
		}
    }
}