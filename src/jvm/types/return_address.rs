use crate::class_loader::parser::U2;

use super::{Value, PrimitiveTypes};

pub struct ReturnAddress {
	value: usize,
}

impl PrimitiveTypes for ReturnAddress {}

impl Value for ReturnAddress {
    type Type = usize;

    fn new() -> Self {
        panic!("Can not create ReturnAddress without a value");
    }

    fn from_value(value: usize) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: usize) {
        self.value = value;
    }

    fn get(&self) -> usize {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}