use crate::class_loader::parser::U2;

use super::{FloatingPointTypes, Value};

pub struct Float {
    value: f32,
}

impl FloatingPointTypes for Float {}

impl Value for Float {
    type Type = f32;

    fn new() -> Self {
        Self { value: 0.0 }
    }

    fn from_value(value: f32) -> Self {
        Self { value }
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

impl Clone for Float {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}
