use crate::{class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfoType}, resolve_constant, jvm::types::reference::Reference};

use super::RuntimeConstant;

#[derive(Clone)]
pub struct StringConstant {
	text: String,
}

impl StringConstant {
	pub fn get(&self) -> Reference {
		unimplemented!("StringConstant::get")
	}
}

impl RuntimeConstant for StringConstant {
	fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		let string = resolve_constant!(ConstantPoolInfoType::String, index, constant_pool);
		let text = resolve_constant!(ConstantPoolInfoType::Utf8, string.string_index, constant_pool).to_string();

		StringConstant {
			text,
		}
	}
}

// FIXME: impl Loadable for StringConstant