use super::Value;

pub struct Long {
	value: i64,
}

impl Value<i64> for Long {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i64) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: i64) {
        self.value = value;
    }

    fn get(&self) -> i64 {
        self.value
    }
}