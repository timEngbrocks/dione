use super::Value;

pub struct Int {
	value: i32,
}

impl Value<i32> for Int {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn from_value(value: i32) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: i32) {
        self.value = value;
    }

    fn get(&self) -> i32 {
        self.value
    }
}