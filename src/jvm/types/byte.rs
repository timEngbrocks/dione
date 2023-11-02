use crate::class_loader::parser::U2;

use super::{Value, IntegralTypes, int::Int};

pub struct Byte {
	value: i8,
}

impl IntegralTypes for Byte {}

impl Byte {
    pub fn to_int(&self) -> Int {
        Int::from_value(self.value as i32)
    }
}

impl Value for Byte {
    type Type = i8;

    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i8) -> Self {
        Self {
            value,
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
            value: self.value,
        }
    }
}