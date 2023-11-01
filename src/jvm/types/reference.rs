use crate::{util::heap::ReferencePtr, class_loader::parser::U2};

use super::Value;

#[derive(Clone)]
pub struct Reference {
	reference: ReferencePtr
}

impl Value for Reference {
    type Type = ReferencePtr;

    fn new() -> Self where Self: Sized {
        Self {
			reference: ReferencePtr::Null,
		}
    }

    fn from_value(value: Self::Type) -> Self where Self: Sized {
        Self {
			reference: value,
		}
    }

    fn set(&mut self, value: Self::Type) {
        match self.reference {
			ReferencePtr::Null => self.reference = value,
			ReferencePtr::Class(_) => self.reference = match value {
				ReferencePtr::Class(_) => value,
				_ => panic!("Invalid reference type"),
			},
			ReferencePtr::Array(_) => self.reference = match value {
				ReferencePtr::Array(_) => value,
				_ => panic!("Invalid reference type"),
			},
			ReferencePtr::Interface(_) => self.reference = match value {
				ReferencePtr::Interface(_) => value,
				_ => panic!("Invalid reference type"),
			},
		}
    }

    fn get(&self) -> Self::Type {
        self.reference.clone()
    }

    fn width(&self) -> U2 {
        1
    }
}

impl Reference {
	pub fn is_null(&self) -> bool {
		match self.reference {
			ReferencePtr::Null => true,
			_ => false,
		}
	}

	pub fn is_class(&self) -> bool {
		match self.reference {
			ReferencePtr::Class(_) => true,
			_ => false,
		}
	}

	pub fn is_array(&self) -> bool {
		match self.reference {
			ReferencePtr::Array(_) => true,
			_ => false,
		}
	}

	pub fn is_interface(&self) -> bool {
		match self.reference {
			ReferencePtr::Interface(_) => true,
			_ => false,
		}
	}
}