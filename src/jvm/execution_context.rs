use crate::{util::{sized_array::SizedArray, stack::Stack}, class_loader::parser::U2};

pub struct ExecutionContext {
	pub local_variables: SizedArray<U2>,
	pub stack: Stack<U2>
}

impl ExecutionContext {
	pub fn new(max_locals: U2, max_stack: U2) -> ExecutionContext {
		ExecutionContext {
			local_variables: SizedArray::<U2>::new(max_locals),
			stack: Stack::<U2>::new(max_stack),
		}
	}
}