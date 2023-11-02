use crate::jvm::runtime_constant_pool::sym_ref_class_or_interface::SymRefClassOrInterface;

use super::{reference::Reference, Types, Value};

pub struct PrimitiveArray {
	kind: Types,
	length: usize,
	data: Vec<Types>,
}

pub enum ReferenceArrayKind {
	Class(SymRefClassOrInterface, Reference),
	Array(SymRefClassOrInterface, Reference),
	Interface(SymRefClassOrInterface, Reference),
}

impl ReferenceArrayKind {
	pub fn new(&self) -> Self {
		match self {
			ReferenceArrayKind::Class(class_ref, _) => ReferenceArrayKind::Class(class_ref.clone(), Reference::new()),
			ReferenceArrayKind::Array(class_ref, _) => ReferenceArrayKind::Array(class_ref.clone(), Reference::new()),
			ReferenceArrayKind::Interface(class_ref, _) => ReferenceArrayKind::Interface(class_ref.clone(), Reference::new()),
		}
	}

	pub fn class_ref(&self) -> &SymRefClassOrInterface {
		match self {
			ReferenceArrayKind::Class(class_ref, _) => class_ref,
			ReferenceArrayKind::Array(class_ref, _) => class_ref,
			ReferenceArrayKind::Interface(class_ref, _) => class_ref,
		}
	}

	pub fn reference(&self) -> &Reference {
		match self {
			ReferenceArrayKind::Class(_, reference) => reference,
			ReferenceArrayKind::Array(_, reference) => reference,
			ReferenceArrayKind::Interface(_, reference) => reference,
		}
	}
}

pub struct ReferenceArray {
	kind: ReferenceArrayKind,
	length: usize,
	data: Vec<ReferenceArrayKind>,
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
	type Type = ReferenceArrayKind;

	fn new(kind: Self::Type, length: usize) -> Self where Self: Sized {
		let mut data = Vec::with_capacity(length);
		match kind {
			ReferenceArrayKind::Class(ref class_ref, _) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Class(class_ref.clone(), Reference::new()));
				}
			},
			ReferenceArrayKind::Array(ref class_ref, _) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Array(class_ref.clone(), Reference::new()));
				}
			},
			ReferenceArrayKind::Interface(ref class_ref, _) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Interface(class_ref.clone(), Reference::new()));
				}
			},
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

impl ReferenceArray {
	pub fn new_preinitialized(kind: ReferenceArrayKind, length: usize) -> Self where Self: Sized {
		let mut data = Vec::with_capacity(length);
		match kind {
			ReferenceArrayKind::Class(ref class_ref, ref reference) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Class(class_ref.clone(), reference.clone()));
				}
			},
			ReferenceArrayKind::Array(ref class_ref, ref reference) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Array(class_ref.clone(), reference.clone()));
				}
			},
			ReferenceArrayKind::Interface(ref class_ref, ref reference) => {
				for _ in 0..length {
					data.push(ReferenceArrayKind::Interface(class_ref.clone(), reference.clone()));
				}
			},
		}
		Self {
			kind,
			length,
			data,
		}
	}
}