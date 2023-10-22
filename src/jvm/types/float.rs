use crate::class_loader::parser::U2;

use super::{Value, FloatingPointTypes};

pub struct Float {
	value: f32,
}

impl FloatingPointTypes for Float {}

impl Value for Float {
    type Type = f32;

    fn new() -> Self {
        Self {
            value: 0.0,
        }
    }

    fn from_value(value: f32) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn get(&self) -> f32 {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}