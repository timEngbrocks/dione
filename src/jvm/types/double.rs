use crate::class_loader::parser::U2;

use super::{FloatingPointTypes, Value};

pub struct Double {
    value: f64,
}

impl FloatingPointTypes for Double {}

impl Value for Double {
    type Type = f64;

    fn new() -> Self {
        Self { value: 0.0 }
    }

    fn from_value(value: f64) -> Self {
        Self { value }
    }

    fn set(&mut self, value: f64) {
        self.value = value;
    }

    fn get(&self) -> f64 {
        self.value
    }

    fn width(&self) -> U2 {
        2
    }
}

impl Clone for Double {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}
