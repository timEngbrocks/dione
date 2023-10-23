use crate::class_loader::parser::U2;

use super::{Value, IntegralTypes};

pub struct Byte {
	value: i8,
}

impl IntegralTypes for Byte {}

impl Value for Byte {
    type Type = i8;

    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i8) -> Self {
        Self {
            value: value,
        }
    }
    
    fn set(&mut self, value: i8) {
        self.value = value;
    }

    fn get(&self) -> i8 {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}


impl Clone for Byte {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}