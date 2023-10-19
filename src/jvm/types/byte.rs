use super::Value;

pub struct Byte {
	value: i8,
}

impl Value<i8> for Byte {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i8) -> Self {
        Self {
            value: value,
        }
    }
    
    fn set(&mut self, value: i8) {
        self.value = value;
    }

    fn get(&self) -> i8 {
        self.value
    }
}