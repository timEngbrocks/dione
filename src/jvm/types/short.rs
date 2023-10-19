use super::Value;

pub struct Short {
	value: i16,
}

impl Value<i16> for Short {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i16) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: i16) {
        self.value = value;
    }

    fn get(&self) -> i16 {
        self.value
    }
}