use crate::class_loader::parser::U2;

use super::{Value, IntegralTypes};

pub struct Long {
	value: i64,
}

impl IntegralTypes for Long {}

impl Value for Long {
    type Type = i64;

    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i64) -> Self {
        Self {
            value,
        }
    }

    fn set(&mut self, value: i64) {
        self.value = value;
    }

    fn get(&self) -> i64 {
        self.value
    }

    fn width(&self) -> U2 {
        2
    }
}

impl Clone for Long {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
        }
    }
}