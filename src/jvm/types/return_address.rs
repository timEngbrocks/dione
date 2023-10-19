use super::Value;

pub struct ReturnAddress {
	value: usize,
}

impl Value<usize> for ReturnAddress {
    fn new() -> Self {
        panic!("Can not create ReturnAddress without a value");
    }

    fn from_value(value: usize) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: usize) {
        self.value = value;
    }

    fn get(&self) -> usize {
        self.value
    }
}