use crate::{util::heap::ReferencePtr, class_loader::parser::U2};

use super::Value;

pub struct Reference {
	value: ReferencePtr,
}

impl Value for Reference {
	type Type = ReferencePtr;

	fn new() -> Self {
		Reference {
			value: ReferencePtr::Null,
		}
	}

	fn from_value(value: ReferencePtr) -> Self {
		Reference {
			value: value,
		}
	}

    fn set(&mut self, value: ReferencePtr) {
        self.value = value;
    }

    fn get(&self) -> ReferencePtr {
        self.value.clone()
    }

	fn width(&self) -> U2 {
        1
    }
}


impl Clone for Reference {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}