use crate::{class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfoType}, resolve_constant};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct SymRefMethodType {
	pub descriptor: String,
}

impl RuntimeConstant for SymRefMethodType {
    fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let method_type = resolve_constant!(ConstantPoolInfoType::MethodType, index, constant_pool);
		let descriptor = resolve_constant!(ConstantPoolInfoType::Utf8, method_type.descriptor_index, constant_pool).to_string();

		SymRefMethodType {
			descriptor,
		}
    }
}

// FIXME: impl Loadable for SymRefMethodType