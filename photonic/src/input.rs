use std::sync::{Arc, Mutex};

pub enum Poll<T> {
    Pending,
    Ready(T),
}

pub struct Input<T> {
    value: Arc<Mutex<Poll<T>>>,
}

pub struct Sink<T> {
    value: Arc<Mutex<Poll<T>>>,
}

impl<T> Input<T> {
    pub fn poll(&mut self) -> Poll<T> {
        let mut value = self.value.lock().unwrap();
        return std::mem::replace(&mut *value, Poll::Pending);
    }

    pub fn new() -> (Self, Sink<T>) {
        let value = Arc::new(Mutex::new(Poll::Pending));

        return (
            Self { value: value.clone() },
            Sink { value: value.clone() },
        );
    }
}

impl<T> Sink<T> {
    pub fn send(&mut self, next: T) {
        let mut value = self.value.lock().unwrap();
        std::mem::replace(&mut *value, Poll::Ready(next));
    }
}
