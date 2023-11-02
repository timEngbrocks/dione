use crate::{class_loader::parser::U2, jvm::types::Width};

pub struct Stack<T> {
	data: Vec<T>,
	elements: U2,
	size: U2,
}

impl <T> Stack<T>
where T: Width
{
	pub fn new(size: U2) -> Stack<T> {
		Stack {
			data: Vec::with_capacity(size as usize),
			elements: 0,
			size,
		}
	}

	pub fn push(&mut self, value: T) {
		if (self.elements + value.width()) > self.size {
			panic!("Error handling!");
		}
		self.elements += value.width();
		self.data.push(value);
	}

	pub fn pop(&mut self) -> T {
		if self.elements == 0 {
			panic!("Error handling!");
		}
		match self.data.pop() {
			Some(v) => {
				self.elements -= v.width();
				v
			},
			None => panic!("Error handling!"),
		}
	}

	pub fn len(&self) -> U2 {
		self.elements
	}

	pub fn is_empty(&self) -> bool {
		self.elements == 0
	}

	pub fn max_size(&self) -> U2 {
		self.size
	}
}
