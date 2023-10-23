use crate::class_loader::parser::U2;

use super::{Value, IntegralTypes};

pub struct Short {
	value: i16,
}

impl IntegralTypes for Short {}

impl Value for Short {
    type Type = i16;

    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i16) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: i16) {
        self.value = value;
    }

    fn get(&self) -> i16 {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}

impl Clone for Short {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}