use super::Value;

pub struct Float {
	value: f32,
}

impl Value<f32> for Float {
    fn new() -> Self {
        Self {
            value: 0.0,
        }
    }

    fn from_value(value: f32) -> Self {
        Self {
            value: value,
        }
    }

    fn set(&mut self, value: f32) {
        self.value = value;
    }

    fn get(&self) -> f32 {
        self.value
    }
}