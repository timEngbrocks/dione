use crate::{class_loader::parser::U2, jvm::types::Width};

pub struct SizedArray<T> {
	data: Vec<DataType<T>>,
	size: U2,
}

enum DataType<T> {
	Value(T),
	Blocked,
	Null,
}

impl <T> SizedArray<T>
where T: Width
{
	pub fn new(size: U2) -> SizedArray<T> {
		let mut data = Vec::with_capacity(size as usize);
		for _ in 0..size {
			data.push(DataType::Null);
		}
		SizedArray {
			data,
			size,
		}
	}

	pub fn set(&mut self, index: U2, value: T) {
		if (index + value.width()) > self.size {
			panic!("Error handling!");
		}
		match self.data.get(index as usize) {
			Some(DataType::Value::<T>(_)) => {
				self.clear_value_from(index);
				self.set_impl(index, value);
			},
			Some(DataType::Blocked) => {
				self.clear_value_from(index);
				let mut offset = 1;
				while (index - offset) > 0 {
					match self.data.get((index - offset) as usize) {
						Some(DataType::Value::<T>(_)) => {
							self.data[(index - offset) as usize] = DataType::Null;
							break;
						},
						Some(DataType::Blocked) => {
							self.data[(index - offset) as usize] = DataType::Null;
						},
						_ => {
							break;
						},
					}
					offset += 1;
				}
				self.set_impl(index, value);
			},
			_ => {
				self.set_impl(index, value);
			},
		}
	}

	fn set_impl(&mut self, index: U2, value: T) {
		let w = value.width();
		self.data[index as usize] = DataType::Value(value);
		for i in 1..w {
			self.data[(index + i) as usize] = DataType::Blocked;
		}
	}

	fn clear_value_from(&mut self, index: U2) {
		let mut offset = 0;
		loop {
			match self.data.get((index + offset) as usize) {
				None | Some(DataType::Null) => {
					break;
				},
				Some(DataType::Value::<T>(_)) if offset > 0 => {
					break;
				},
				Some(_) => {
					self.data[(index + offset) as usize] = DataType::Null;
				}
			}
			offset += 1;
		}
	}

	pub fn get(&mut self, index: U2) -> &T {
		if index >= self.size {
			panic!("Error handling!");
		}
		match self.data.get(index as usize) {
			Some(DataType::Value::<T>(value)) => value,
			_ => panic!("Error handling!"),
		}
	}

	pub fn len(&self) -> U2 {
		self.size
	}
}