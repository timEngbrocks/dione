use super::Value;

pub struct Char {
	value: u16,
}

impl Value<u16> for Char {
    fn new() -> Self {
        Self {
            value: '\0' as u16,
        }
    }

    fn from_value(value: u16) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: u16) {
        self.value = value;
    }

    fn get(&self) -> u16 {
        self.value
    }
}