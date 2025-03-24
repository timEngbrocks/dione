use crate::numbers::{MAX, Width};

pub struct SizedArray<S, T>
where
    S: Into<usize> + MAX,
{
    data: Vec<DataType<T>>,
    size: S,
}

enum DataType<T> {
    Value(T),
    Blocked,
    Null,
}

impl<S, T> SizedArray<S, T>
where
    S: Into<usize> + MAX + Copy,
    T: Width,
{
    pub fn new(size: Option<S>) -> SizedArray<S, T> {
        if let Some(size) = size {
            SizedArray::new_impl(size)
        } else {
            SizedArray::new_dynamic_impl()
        }
    }

    pub fn set(&mut self, index: S, value: T) {
        if (index.into() + value.width() - 1) >= self.size.into() {
            panic!(
                "Error handling!, {}, {}, {}",
                index.into(),
                value.width(),
                self.size.into()
            );
        }
        match self.data.get(index.into()) {
            Some(DataType::Value::<T>(_)) => {
                self.clear_value_from(index);
                self.set_impl(index, value);
            }
            Some(DataType::Blocked) => {
                self.clear_value_from(index);
                let mut offset = 1;
                while (index.into() - offset) > 0 {
                    match self.data.get(index.into() - offset) {
                        Some(DataType::Value::<T>(_)) => {
                            self.data[index.into() - offset] = DataType::Null;
                            break;
                        }
                        Some(DataType::Blocked) => {
                            self.data[index.into() - offset] = DataType::Null;
                        }
                        _ => {
                            break;
                        }
                    }
                    offset += 1;
                }
                self.set_impl(index, value);
            }
            _ => {
                self.set_impl(index, value);
            }
        }
    }

    pub fn map<F, R>(&self, index: S, mapper: F) -> R
    where
        F: Fn(&T) -> R,
    {
        let element = self.get(index);
        mapper(element)
    }

    fn set_impl(&mut self, index: S, value: T) {
        let w = value.width();
        if self.size.into() == S::max_value().into() {
            if self.data.len() < index.into() + w {
                self.data.reserve((index.into() + w) - self.data.len());
            }
            for i in 0..(index.into() + w) {
                if self.data.get(i).is_none() {
                    self.data.push(DataType::Null);
                }
            }
        }
        self.data[index.into()] = DataType::Value(value);
        for i in 1..w {
            self.data[index.into() + i] = DataType::Blocked;
        }
    }

    fn clear_value_from(&mut self, index: S) {
        let mut offset = 0;
        loop {
            match self.data.get(index.into() + offset) {
                None | Some(DataType::Null) => {
                    break;
                }
                Some(DataType::Value::<T>(_)) if offset > 0 => {
                    break;
                }
                Some(_) => {
                    self.data[index.into() + offset] = DataType::Null;
                }
            }
            offset += 1;
        }
    }

    pub fn get(&self, index: S) -> &T {
        if index.into() >= self.size.into() {
            panic!("Error handling!");
        }
        match self.data.get(index.into()) {
            Some(DataType::Value::<T>(value)) => value,
            _ => panic!("Error handling!"),
        }
    }

    pub fn len(&self) -> Option<S> {
        if self.size.into() == S::max_value().into() {
            None
        } else {
            Some(self.size)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size.into() == 0
    }

    fn new_impl(size: S) -> SizedArray<S, T> {
        let mut data = Vec::with_capacity(size.into());
        for _ in 0..size.into() {
            data.push(DataType::Null);
        }
        SizedArray { data, size }
    }

    fn new_dynamic_impl() -> SizedArray<S, T> {
        SizedArray {
            data: Vec::new(),
            size: S::max_value(),
        }
    }
}
