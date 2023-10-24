use crate::{util::{sized_array::SizedArray, stack::Stack}, class_loader::constant_pool_info::ConstantPool, jvm::types::Types};

use super::{runtime_constant_pool::RuntimeConstantPool, types::ReturnTypes};

pub struct Frame {
	pub local_variables: SizedArray<Types>,
	pub stack: Stack<Types>,
	pub runtime_constant_pool: RuntimeConstantPool,
	pub method_name: String,
	pub return_value: ReturnTypes,
}

impl Frame {
	pub fn new(
		local_variables: SizedArray<Types>,
		stack: Stack<Types>,
		constant_pool: &ConstantPool,
		method_name: String,
		return_value: ReturnTypes
	) -> Frame {
		Frame {
			local_variables,
			stack,
			runtime_constant_pool: RuntimeConstantPool::new(constant_pool),
			method_name,
			return_value,
		}
	}

	pub fn get_return_value(&self) -> &ReturnTypes {
		&self.return_value
	}

	pub fn set_return_from_called_method(&mut self, return_value: ReturnTypes) {
		self.return_value = return_value;
	}
}

impl Clone for Frame {
	fn clone(&self) -> Frame {
		Frame {
			local_variables: SizedArray::<Types>::new(self.local_variables.len()),
			stack: Stack::<Types>::new(self.stack.max_size()),
			runtime_constant_pool: self.runtime_constant_pool.clone(),
			method_name: self.method_name.clone(),
			return_value: self.return_value.clone(),
		}
	}
}