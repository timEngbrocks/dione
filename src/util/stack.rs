use crate::class_loader::parser::U2;

pub struct Stack<T> {
	data: Vec<T>,
	elements: U2,
	size: U2,
}

impl <T> Stack<T> {
	pub fn new(size: U2) -> Stack<T> {
		Stack {
			data: Vec::with_capacity(size as usize),
			elements: 0,
			size,
		}
	}

	pub fn push(&mut self, value: T) {
		if self.elements == self.size {
			panic!("Error handling!");
		}
		self.data.push(value);
		self.elements += 1;
	}

	pub fn pop(&mut self) -> T {
		if self.elements == 0 {
			panic!("Error handling!");
		}
		self.elements -= 1;
		match self.data.pop() {
			Some(v) => v,
			None => panic!("Error handling!"),
		}
	}

	pub fn len(&self) -> U2 {
		self.elements
	}
}
