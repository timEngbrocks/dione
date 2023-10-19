use core::panic;

use super::Value;

pub struct Boolean {
	value: u8,
}

#[repr(u8)]
pub enum BooleanValue {
    False = 0,
    True = 1
}

impl Value<BooleanValue> for Boolean {
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
}