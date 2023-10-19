use crate::util::heap::ReferencePtr;

use super::Value;

pub struct Reference {
	value: ReferencePtr,
}

impl Value<ReferencePtr> for Reference {
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
}