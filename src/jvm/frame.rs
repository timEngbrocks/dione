use crate::{util::{sized_array::SizedArray, stack::Stack}, class_loader::{parser::U2, constant_pool_info::ConstantPool}, jvm::types::Types};

use super::runtime_constant_pool::RuntimeConstantPool;

pub struct Frame {
	pub local_variables: SizedArray<Types>,
	pub stack: Stack<Types>,
	pub constant_pool: RuntimeConstantPool,
}

impl Frame {
	pub fn new_native(constant_pool: &ConstantPool) -> Frame {
		Frame {
			local_variables: SizedArray::<Types>::new(0),
			stack: Stack::<Types>::new(0),
			constant_pool: RuntimeConstantPool::new(constant_pool),
		}
	}

	pub fn new(max_locals: U2, max_stack: U2, constant_pool: &ConstantPool) -> Frame {
		Frame {
			local_variables: SizedArray::<Types>::new(max_locals),
			stack: Stack::<Types>::new(max_stack),
			constant_pool: RuntimeConstantPool::new(constant_pool),
		}
	}

	pub fn clone(&self) -> Frame {
		Frame {
			local_variables: SizedArray::<Types>::new(self.local_variables.len()),
			stack: Stack::<Types>::new(self.stack.max_size()),
			constant_pool: self.constant_pool.clone(),
		}
	}
}