use super::{reference::Reference, Types, Value};

pub struct PrimitiveArray {
	kind: Types,
	length: usize,
	data: Vec<Types>,
}

pub struct ReferenceArray {
	kind: Reference,
	length: usize,
	data: Vec<Reference>,
}

pub trait Array {
	type Type;

	fn new(kind: Self::Type, length: usize) -> Self where Self: Sized;
	fn get(&self, index: usize) -> &Self::Type;
	fn get_all(&self) -> &Vec<Self::Type>;
	fn set(&mut self, index: usize, value: Self::Type);
	fn set_all(&mut self, values: Vec<Self::Type>);
	fn length(&self) -> usize;
	fn kind(&self) -> &Self::Type;
}

impl Array for PrimitiveArray {
	type Type = Types;

	fn new(kind: Self::Type, length: usize) -> Self where Self: Sized {
		match kind {
			Types::Reference(_) |
			Types::ReturnAddress(_) => panic!("Invalid primitive array type"),
			_ => (),
		};
		let mut data = Vec::with_capacity(length);
		for _ in 0..length {
			data.push(kind.new());
		}
		Self {
			kind,
			length,
			data,
		}
	}

	fn get(&self, index: usize) -> &Self::Type {
		&self.data[index]
	}

	fn get_all(&self) -> &Vec<Self::Type> {
		&self.data
	}

	fn set(&mut self, index: usize, value: Self::Type) {
		self.data[index] = value;
	}

	fn set_all(&mut self, values: Vec<Self::Type>) {
		self.data = values;
	}

	fn length(&self) -> usize {
		self.length
	}

	fn kind(&self) -> &Self::Type {
		&self.kind
	}
}

impl Array for ReferenceArray {
	type Type = Reference;

	fn new(kind: Self::Type, length: usize) -> Self where Self: Sized {
		let mut data = Vec::with_capacity(length);
		for _ in 0..length {
			data.push(Reference::new());
		}
		Self {
			kind,
			length,
			data,
		}
	}

	fn get(&self, index: usize) -> &Self::Type {
		&self.data[index]
	}

	fn get_all(&self) -> &Vec<Self::Type> {
		&self.data
	}

	fn set(&mut self, index: usize, value: Self::Type) {
		self.data[index] = value;
	}

	fn set_all(&mut self, values: Vec<Self::Type>) {
		self.data = values;
	}

	fn length(&self) -> usize {
		self.length
	}

	fn kind(&self) -> &Self::Type {
		&self.kind
	}
}