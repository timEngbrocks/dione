use crate::numbers::{MAX, Width, ZERO};

pub struct Stack<S, T>
where
    S: Into<usize> + MAX,
{
    data: Vec<T>,
    elements: S,
    size: S,
}

impl<S, T> Stack<S, T>
where
    S: Into<usize> + From<usize> + MAX + ZERO + Copy,
    T: Width,
{
    pub fn new(size: Option<S>) -> Stack<S, T> {
        if let Some(size) = size {
            Stack::new_impl(size)
        } else {
            Stack::new_dynamic_impl()
        }
    }

    pub fn push(&mut self, value: T) {
        if (self.elements.into() + value.width()) > self.size.into() {
            panic!("Error handling!");
        }
        self.elements = (self.elements.into() + value.width()).into();
        self.data.push(value);
    }

    pub fn pop(&mut self) -> T {
        if self.elements.into() == 0 {
            panic!("Error handling!");
        }
        match self.data.pop() {
            Some(v) => {
                self.elements = (self.elements.into() - v.width()).into();
                v
            }
            None => panic!("Error handling!"),
        }
    }

    pub fn len(&self) -> S {
        self.elements
    }

    pub fn is_empty(&self) -> bool {
        self.elements.into() == 0
    }

    pub fn max_size(&self) -> Option<S> {
        if self.size.into() == S::max_value().into() {
            None
        } else {
            Some(self.size)
        }
    }

    fn new_impl(size: S) -> Stack<S, T> {
        Stack {
            data: Vec::with_capacity(size.into()),
            elements: S::zero_value(),
            size,
        }
    }

    fn new_dynamic_impl() -> Stack<S, T> {
        Stack {
            data: Vec::new(),
            elements: S::zero_value(),
            size: S::max_value(),
        }
    }
}
