use crate::class_loader::parser::U2;

use super::{int::Int, IntegralTypes, Value};

pub struct Char {
    value: u16,
}

impl Char {
    pub fn to_int(&self) -> Int {
        Int::from_value(self.value as i32)
    }
}

impl IntegralTypes for Char {}

impl Value for Char {
    type Type = u16;

    fn new() -> Self {
        Self { value: '\0' as u16 }
    }

    fn from_value(value: u16) -> Self {
        Self { value }
    }

    fn set(&mut self, value: u16) {
        self.value = value;
    }

    fn get(&self) -> u16 {
        self.value
    }

    fn width(&self) -> U2 {
        1
    }
}

impl Clone for Char {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}
