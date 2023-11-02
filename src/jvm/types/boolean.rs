use core::panic;

use crate::class_loader::parser::U2;

use super::{Value, PrimitiveTypes};

pub struct Boolean {
	value: u8,
}

impl PrimitiveTypes for Boolean {}

#[repr(u8)]
pub enum BooleanValue {
    False = 0,
    True = 1
}

impl BooleanValue {
    pub fn from_value(value: i32) -> Self {
        match value {
            0 => BooleanValue::False,
            1 => BooleanValue::True,
            _ => panic!("Invalid boolean value: {}", value)
        }
    }
}

impl Value for Boolean {
    type Type = BooleanValue;

    fn new() -> Self {
        Self {
            value: BooleanValue::False as u8,
        }
    }

    fn from_value(value: BooleanValue) -> Self {
        Self {
            value: value as u8
        }
    }

    fn set(&mut self, value: BooleanValue) {
        self.value = value as u8;
    }

    fn get(&self) -> BooleanValue {
        match self.value {
            0 => BooleanValue::False,
            1 => BooleanValue::True,
            v => panic!("Invalid boolean value: {}", v)
        }
    }

    fn width(&self) -> U2 {
        1
    }
}

impl Clone for Boolean {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}