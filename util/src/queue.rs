pub struct Queue<V> {
    is_fifo: bool,
    values: Vec<V>,
}

impl<V> Queue<V> {
    pub fn new(is_fifo: bool) -> Self {
        Queue {
            is_fifo,
            values: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: V) {
        if self.is_fifo {
            self.values.push(value);
        } else {
            self.values.insert(0, value);
        }
    }

    pub fn dequeue(&mut self) -> Option<V> {
        if self.is_empty() {
            return None;
        }

        if self.is_fifo {
            Some(self.values.remove(0))
        } else {
            self.values.pop()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
