use crate::{resolve_constant, jvm::runtime_constant_pool::ConstantPool, class_loader::constant_pool_info::ConstantPoolInfoType};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct RuntimeConstantFloat {
	pub value: f32,
}

impl RuntimeConstant for RuntimeConstantFloat {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let value = resolve_constant!(ConstantPoolInfoType::Float, index, constant_pool).to_f32();

		RuntimeConstantFloat {
			value,
		}
    }
}