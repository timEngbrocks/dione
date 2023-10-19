use crate::class_loader::parser::U2;

pub struct SizedArray<T> {
	data: Vec<T>,
	size: U2,
}

impl <T> SizedArray<T> {
	pub fn new(size: U2) -> SizedArray<T> {
		SizedArray {
			data: Vec::with_capacity(size as usize),
			size,
		}
	}

	pub fn set(&mut self, index: U2, value: T) {
		if index >= self.size {
			panic!("Error handling!");
		}
		self.data[index as usize] = value;
	}

	pub fn get(&mut self, index: U2) -> &T {
		if index >= self.size {
			panic!("Error handling!");
		}
		&self.data[index as usize]
	}

	pub fn len(&self) -> U2 {
		self.size
	}
}