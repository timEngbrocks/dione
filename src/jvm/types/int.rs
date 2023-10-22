use crate::class_loader::parser::U2;

use super::{Value, IntegralTypes};

pub struct Int {
	value: i32,
}

impl IntegralTypes for Int {}

impl Value for Int {
    type Type = i32;
    
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i32) -> Self {
        Self {
            value,
        }
    }

    fn set(&mut self, value: i32) {
        self.value = value;
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}