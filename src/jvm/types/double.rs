use super::Value;

pub struct Double {
	value: f64,
}

impl Value<f64> for Double {
    fn new() -> Self {
        Self {
            value: 0.0,
        }
    }

    fn from_value(value: f64) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: f64) {
        self.value = value;
    }

    fn get(&self) -> f64 {
        self.value
    }
}